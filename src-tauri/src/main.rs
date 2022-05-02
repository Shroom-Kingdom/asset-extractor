#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod bundle;
mod error;
mod keys;
mod mods;
mod ninres;
mod xci;

pub type Result<T> = std::result::Result<T, error::Error>;

use bundle::{bundle_assets, find_romfs_dir, find_romfs_dir_in_zip_archive, finish_bundle_assets};
use error::Error;
use glob::glob;
use itertools::Itertools;
use mods::{extract_7z, extract_zip};
use nfd2::Response;
use pathdiff::diff_paths;
use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    fs::{self, File},
    io,
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
    file_content: RwLock<HashMap<PathBuf, Vec<String>>>,
    required_files: RwLock<Vec<String>>,
    has_original_game_files: RwLock<bool>,
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            keys: RwLock::new(vec![]),
            prod_key: RwLock::new(None),
            selected_files: Arc::new(RwLock::new(vec![])),
            bundle_data: RwLock::new(None),
            file_content: RwLock::new(HashMap::new()),
            required_files: RwLock::new(vec![
                "romfs/Pack/MW_Model.pack".to_string(),
                "romfs/Model/MW_Field_plain.Nin_NX_NVN.zs".to_string(),
                "romfs/Model/MW_DV_plain_V.Nin_NX_NVN.zs".to_string(),
                "romfs/Model/MW_Field_underground.Nin_NX_NVN.zs".to_string(),
                "romfs/Model/MW_DV_underground_V.Nin_NX_NVN.zs".to_string(),
                "romfs/Model/MW_Field_water.Nin_NX_NVN.zs".to_string(),
                "romfs/Model/MW_DV_water_V.Nin_NX_NVN.zs".to_string(),
                "romfs/Model/MW_Field_hauntedhouse.Nin_NX_NVN.zs".to_string(),
                "romfs/Model/MW_DV_hauntedhouse_V.Nin_NX_NVN.zs".to_string(),
                "romfs/Model/MW_Field_castle.Nin_NX_NVN.zs".to_string(),
                "romfs/Model/MW_DV_castle_V.Nin_NX_NVN.zs".to_string(),
                "romfs/Model/MW_Field_woods.Nin_NX_NVN.zs".to_string(),
                "romfs/Model/MW_DV_woods_V.Nin_NX_NVN.zs".to_string(),
                "romfs/Model/MW_Field_desert.Nin_NX_NVN.zs".to_string(),
                "romfs/Model/MW_DV_desert_V.Nin_NX_NVN.zs".to_string(),
                "romfs/Model/MW_Field_snow.Nin_NX_NVN.zs".to_string(),
                "romfs/Model/MW_DV_snow_V.Nin_NX_NVN.zs".to_string(),
                "romfs/Model/MW_Field_airship.Nin_NX_NVN.zs".to_string(),
                "romfs/Model/MW_DV_airship_V.Nin_NX_NVN.zs".to_string(),
            ]),
            has_original_game_files: RwLock::new(false),
        })
        .invoke_handler(tauri::generate_handler![
            find_keys,
            set_prod_key,
            select_prod_key,
            add_files,
            assert_added_files,
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
async fn add_files(
    files: Vec<PathBuf>,
    state: State<'_, AppState>,
    window: Window,
) -> Result<Vec<PathBuf>> {
    for file in files {
        let window = window.clone();
        check_added_file(&state, &file, window).await?;
        state.selected_files.write().unwrap().push(file);
    }
    dedup_files(&state);
    Ok(state.selected_files.read().unwrap().clone())
}

fn dedup_files(state: &State<AppState>) {
    let files = state
        .selected_files
        .read()
        .unwrap()
        .iter()
        .cloned()
        .unique()
        .collect();
    *state.selected_files.write().unwrap() = files;
}

async fn check_added_file(state: &State<'_, AppState>, file: &Path, window: Window) -> Result<()> {
    let file_name = Path::new(file);
    let extension = file_name.extension().and_then(OsStr::to_str);
    if extension == Some("zip") {
        let file = File::open(file).unwrap();

        let mut archive = zip::ZipArchive::new(file).unwrap();
        let romfs_dir = find_romfs_dir_in_zip_archive(&mut archive)?;

        let mut file_content = vec![];
        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            if let Some(name) = file.enclosed_name() {
                let mut parent_dir = romfs_dir.clone();
                parent_dir.pop();
                if let Some(path_diff) = diff_paths(&name, parent_dir) {
                    let name = path_diff.to_string_lossy().to_string();
                    if state.required_files.read().unwrap().contains(&name) {
                        file_content.push(name);
                    }
                }
            }
        }
        state
            .file_content
            .write()
            .unwrap()
            .insert(file_name.to_path_buf(), file_content);
    } else if extension == Some("7z") {
        let dir = tempdir()?;
        extract_7z(window, &dir, file).await?;

        let mut file_content = vec![];
        let romfs_dir = find_romfs_dir(&dir)?;
        for entry in glob(&format!("{}/**/*", dir.path().display())).unwrap() {
            let entry = entry?;
            let mut parent_dir = romfs_dir.clone();
            parent_dir.pop();
            if let Some(path_diff) = diff_paths(&entry, parent_dir) {
                let name = path_diff.to_string_lossy().to_string();
                if state.required_files.read().unwrap().contains(&name) {
                    file_content.push(name);
                }
            }
        }
        state
            .file_content
            .write()
            .unwrap()
            .insert(file_name.to_path_buf(), file_content);
    } else if extension == Some("xci") || extension == Some("nsp") {
        *state.has_original_game_files.write().unwrap() = true;
    }
    Ok(())
}

#[tauri::command]
fn assert_added_files(state: State<AppState>) -> Result<()> {
    if *state.has_original_game_files.read().unwrap() {
        return Ok(());
    }

    let mut err_res = vec![];
    let state_files = state.required_files.read().unwrap();
    let mut required_files = {
        let mut map = HashMap::new();
        for required_file in state_files.iter() {
            map.insert(required_file, false);
        }
        map
    };
    let file_contents = state.file_content.read().unwrap();
    for file_content in file_contents.values() {
        for file in file_content.iter() {
            if required_files.contains_key(file) {
                required_files.insert(file, true);
            }
        }
    }
    required_files.into_iter().for_each(|(k, v)| {
        if !v {
            err_res.push(k.clone());
        }
    });
    if err_res.is_empty() {
        Ok(())
    } else {
        Err(Error::RequiredFilesMissing(err_res))
    }
}

#[tauri::command]
fn remove_file(file_name: PathBuf, state: State<AppState>) -> Vec<PathBuf> {
    let mut has_original_game_files = false;
    let extension = file_name.extension().and_then(OsStr::to_str);
    state.selected_files.write().unwrap().retain(|f| {
        if extension == Some("xci") || extension == Some("nsp") {
            has_original_game_files = true;
        }
        f != &file_name
    });
    *state.has_original_game_files.write().unwrap() = has_original_game_files;
    state.file_content.write().unwrap().remove(&file_name);
    state.selected_files.read().unwrap().clone()
}

#[tauri::command]
async fn extract_assets(state: State<'_, AppState>, window: Window) -> Result<()> {
    let files = state.selected_files.read().unwrap().clone();
    let prod_key = state.prod_key.read().unwrap().clone();
    let mut prod_key_required = false;
    let progress = Arc::new(RwLock::new(0f64));
    let max_progress = files.iter().fold(3u32, |acc, file| {
        let file_name = file.to_string_lossy();
        if file_name.ends_with(".xci") {
            prod_key_required = true;
            acc + 2
        } else if file_name.ends_with(".nsp") {
            prod_key_required = true;
            acc + 1
        } else {
            acc
        }
    });
    if prod_key_required && prod_key.is_none() {
        return Err(Error::ProdKeyNotSet);
    }

    let cursor = io::Cursor::new(vec![]);
    let builder = RwLock::new(tar::Builder::new(cursor));

    for (index, file) in files.iter().enumerate() {
        let dir = tempdir()?;

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
        let extension = Path::new(file_name.as_ref())
            .extension()
            .and_then(OsStr::to_str);
        let romfs_dir = if extension == Some("xci") {
            let romfs_dir = dir.path().join("romfs");
            let exefs_dir = dir.path().join("exefs");
            extract_xci(
                window.clone(),
                &dir,
                &romfs_dir,
                &exefs_dir,
                file,
                prod_key.as_ref().unwrap(),
                progress.clone(),
                max_progress,
                &file_message,
            )
            .await?;
            romfs_dir
        } else if extension == Some("nsp") {
            // TODO
            todo!();
        } else if extension == Some("zip") {
            extract_zip(&dir, file);
            find_romfs_dir(&dir)?
        } else if extension == Some("7z") {
            extract_7z(window.clone(), &dir, file).await?;
            find_romfs_dir(&dir)?
        } else {
            return Err(error::Error::FileExtensionUnsupported);
        };
        bundle_assets(
            window.clone(),
            &builder,
            &dir,
            &romfs_dir,
            progress.clone(),
            max_progress,
            &file_message,
        )?;
    }

    let file_message = format!(
        "[{}/{}] All files extracted",
        files.len() + 1,
        files.len() + 1,
    );
    *state.bundle_data.write().unwrap() = Some(finish_bundle_assets(
        window.clone(),
        builder,
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
