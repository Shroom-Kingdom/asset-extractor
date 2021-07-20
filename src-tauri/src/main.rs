#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod error;
mod ninres;
mod xci;

pub type Result<T> = std::result::Result<T, error::Error>;

pub use crate::ninres::extract_ninres;

use itertools::Itertools;
use nfd2::Response;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tauri::{State, Window};
use xci::extract_xci;

struct SelectedFiles(Arc<RwLock<Vec<PathBuf>>>);

fn main() {
    tauri::Builder::default()
        .manage(SelectedFiles(Arc::new(RwLock::new(vec![]))))
        .invoke_handler(tauri::generate_handler![
            add_files,
            remove_file,
            extract_assets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn add_files(state: State<SelectedFiles>) -> Result<Vec<PathBuf>> {
    let result = nfd2::dialog_multiple().filter("xci,nsp").open()?;

    match result {
        Response::Okay(file_path) => {
            state.0.write().unwrap().push(file_path);
            let files = state.0.read().unwrap().iter().cloned().unique().collect();
            *state.0.write().unwrap() = files;
            Ok(state.0.read().unwrap().clone())
        }
        Response::OkayMultiple(files) => {
            for file in files {
                state.0.write().unwrap().push(file);
            }
            let files = state.0.read().unwrap().iter().cloned().unique().collect();
            *state.0.write().unwrap() = files;
            Ok(state.0.read().unwrap().clone())
        }
        Response::Cancel => Err(error::Error::FileSelectCanceled),
    }
}

#[tauri::command]
fn remove_file(file_name: String, state: State<SelectedFiles>) -> Vec<PathBuf> {
    state
        .0
        .write()
        .unwrap()
        .retain(|f| f.to_string_lossy() != file_name);
    state.0.read().unwrap().clone()
}

#[tauri::command]
async fn extract_assets(state: State<'_, SelectedFiles>, window: Window) -> Result<()> {
    let files = state.0.read().unwrap().clone();
    let progress = Arc::new(RwLock::new(0));
    let max_progress = files.iter().fold(1u32, |acc, file| {
        let file_name = file.to_string_lossy();
        if file_name.ends_with(".xci") {
            acc + 2
        } else if file_name.ends_with(".nsp") {
            acc + 1
        } else {
            acc
        }
    });
    for (index, file) in files.iter().enumerate() {
        let file_name = file.to_string_lossy();
        let file_message = format!(
            "[{}/{}] Processing file {}",
            index + 1,
            files.len(),
            file_name
        );
        window.emit(
            "extract_step",
            format!("{}\nExtracting XCI...", file_message),
        )?;
        if file_name.ends_with(".xci") {
            extract_xci(
                window.clone(),
                file,
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

    Ok(())
}
