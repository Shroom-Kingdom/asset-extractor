#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod bundle;
mod error;
mod keys;
mod ninres;
mod xci;

pub type Result<T> = std::result::Result<T, error::Error>;

use bundle::bundle_assets;
use itertools::Itertools;
use nfd2::Response;
use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, RwLock},
};
use tauri::{State, Window};
use tempfile::tempdir;
use xci::extract_xci;

struct AppState {
    keys: RwLock<Vec<PathBuf>>,
    prod_key: RwLock<Option<PathBuf>>,
    selected_files: Arc<RwLock<Vec<PathBuf>>>,
    bundle_data: RwLock<Option<Vec<u8>>>,
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            keys: RwLock::new(vec![]),
            prod_key: RwLock::new(None),
            selected_files: Arc::new(RwLock::new(vec![])),
            bundle_data: RwLock::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            find_keys,
            set_prod_key,
            select_prod_key,
            add_files,
            remove_file,
            extract_assets,
            save_bundle_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn find_keys(state: State<AppState>) -> Result<Vec<PathBuf>> {
    let keys = keys::find_keys()?;
    *state.keys.write().unwrap() = keys.clone();
    Ok(keys)
}

#[tauri::command]
fn set_prod_key(prod_key: PathBuf, state: State<AppState>) {
    *state.prod_key.write().unwrap() = Some(prod_key);
}

#[tauri::command]
fn select_prod_key(state: State<AppState>) -> Result<PathBuf> {
    let result = nfd2::dialog().open()?;

    match result {
        Response::Okay(file_path) => {
            *state.prod_key.write().unwrap() = Some(file_path);
            Ok(state.prod_key.read().unwrap().clone().unwrap())
        }
        Response::OkayMultiple(_) => {
            unreachable!();
        }
        Response::Cancel => Err(error::Error::FileSelectCanceled),
    }
}

#[tauri::command]
fn add_files(state: State<AppState>) -> Result<Vec<PathBuf>> {
    let result = nfd2::dialog_multiple().filter("xci,nsp").open()?;

    match result {
        Response::Okay(file_path) => {
            state.selected_files.write().unwrap().push(file_path);
            let files = state
                .selected_files
                .read()
                .unwrap()
                .iter()
                .cloned()
                .unique()
                .collect();
            *state.selected_files.write().unwrap() = files;
            Ok(state.selected_files.read().unwrap().clone())
        }
        Response::OkayMultiple(files) => {
            for file in files {
                state.selected_files.write().unwrap().push(file);
            }
            let files = state
                .selected_files
                .read()
                .unwrap()
                .iter()
                .cloned()
                .unique()
                .collect();
            *state.selected_files.write().unwrap() = files;
            Ok(state.selected_files.read().unwrap().clone())
        }
        Response::Cancel => Err(error::Error::FileSelectCanceled),
    }
}

#[tauri::command]
fn remove_file(file_name: String, state: State<AppState>) -> Vec<PathBuf> {
    state
        .selected_files
        .write()
        .unwrap()
        .retain(|f| f.to_string_lossy() != file_name);
    state.selected_files.read().unwrap().clone()
}

#[tauri::command]
async fn extract_assets(state: State<'_, AppState>, window: Window) -> Result<()> {
    let files = state.selected_files.read().unwrap().clone();
    let prod_key = state
        .prod_key
        .read()
        .unwrap()
        .clone()
        .ok_or(error::Error::ProdKeyNotSet)?;
    let progress = Arc::new(RwLock::new(0f64));
    let max_progress = files.iter().fold(3u32, |acc, file| {
        let file_name = file.to_string_lossy();
        if file_name.ends_with(".xci") {
            acc + 2
        } else if file_name.ends_with(".nsp") {
            acc + 1
        } else {
            acc
        }
    });

    let dir = tempdir()?;
    let romfs_dir = dir.path().join("romfs");
    let exefs_dir = dir.path().join("exefs");

    for (index, file) in files.iter().enumerate() {
        let file_name = file.to_string_lossy();
        let file_message = format!(
            "[{}/{}] Processing file {}",
            index + 1,
            files.len() + 1,
            file_name
        );
        window.emit(
            "extract_step",
            format!("{}\nExtracting XCI...", file_message),
        )?;
        let window = window.clone();
        if file_name.ends_with(".xci") {
            extract_xci(
                window,
                &dir,
                &romfs_dir,
                &exefs_dir,
                file,
                &prod_key,
                progress.clone(),
                max_progress,
                &file_message,
            )
            .await?;
        } else if file_name.ends_with(".nsp") {
        } else {
            return Err(error::Error::FileExtensionUnsupported);
        }
    }

    let file_message = format!(
        "[{}/{}] All files extracted",
        files.len() + 1,
        files.len() + 1,
    );
    *state.bundle_data.write().unwrap() = Some(bundle_assets(
        window.clone(),
        dir,
        romfs_dir,
        progress.clone(),
        max_progress,
        &file_message,
    )?);

    Ok(())
}

#[tauri::command]
fn save_bundle_data(state: State<AppState>) -> Result<()> {
    let home = env::var("HOME").ok().unwrap_or_default();
    let default_path = if let "" = home.as_ref() {
        None
    } else {
        Some(Path::new(&home))
    };
    let result = nfd2::open_save_dialog(None, default_path)?;

    match result {
        Response::Okay(mut file_path) => {
            if file_path.file_name().is_none() {
                file_path.set_file_name("shroom_kingdom_assets");
            }
            if file_path.extension().is_none() || file_path.extension().unwrap() != ".tar" {
                file_path.set_extension("tar");
            }
            if let Some(bundle_data) = &*state.bundle_data.read().unwrap() {
                fs::write(file_path, bundle_data)?;
            }
            Ok(())
        }
        Response::OkayMultiple(_) | Response::Cancel => Err(error::Error::FileSelectCanceled),
    }
}

pub fn increase_progress(
    window: Window,
    progress: Arc<RwLock<f64>>,
    max_progress: u32,
) -> Result<()> {
    let p = *progress.read().unwrap() + 1.;
    *progress.write().unwrap() = p;
    let extract_progress = (p as f64 / max_progress as f64) * 100.;
    window.emit("extract_progress", extract_progress)?;
    Ok(())
}

pub fn increase_progress_sync(
    window: Arc<Mutex<Window>>,
    progress: Arc<RwLock<f64>>,
    max_progress: u32,
) -> Result<()> {
    let p = *progress.read().unwrap() + 1.;
    *progress.write().unwrap() = p;
    let extract_progress = (p as f64 / max_progress as f64) * 100.;
    window
        .lock()
        .unwrap()
        .emit("extract_progress", extract_progress)?;
    Ok(())
}
