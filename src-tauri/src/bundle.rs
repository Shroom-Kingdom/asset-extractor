use crate::{increase_progress_sync, ninres::bundle_ninres, Result};
use ninres::NinRes;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{
    fs::{self, read_dir, DirEntry, ReadDir},
    io::Cursor,
    path::PathBuf,
    sync::{Arc, Mutex, RwLock},
    time::SystemTime,
};
use tauri::Window;
use tempfile::TempDir;

pub fn bundle_assets(
    window: Window,
    dir: TempDir,
    romfs_dir: PathBuf,
    progress: Arc<RwLock<f64>>,
    max_progress: u32,
    file_message: &str,
) -> Result<Vec<u8>> {
    window
        .emit("extract_step", &format!("{}\nBundling...", file_message))
        .unwrap();

    let ninres_dir = dir.path().join("ninres");
    let model_dir = romfs_dir.join("Model");
    let pack_dir = romfs_dir.join("Pack");

    let cursor = Cursor::new(vec![]);
    let builder = RwLock::new(tar::Builder::new(cursor));
    let mtime = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut max_completed = 0u32;
    let completed = Arc::new(RwLock::new(0u32));
    let start_progress = *progress.read().unwrap();

    read_dir(model_dir.clone())
        .into_iter()
        .flatten()
        .for_each(|_| max_completed += 1);
    read_dir(pack_dir.clone())
        .into_iter()
        .flatten()
        .for_each(|_| max_completed += 1);

    let window = Arc::new(Mutex::new(window));
    read_dir(model_dir)
        .into_iter()
        .flatten()
        .chain(read_dir(pack_dir).into_iter().flatten())
        .par_bridge()
        .map(|dir_entry| -> Result<_> {
            let dir_entry: DirEntry = dir_entry?;
            let line = format!(
                "Bundling {:?}",
                dir_entry.path().file_name().unwrap_or_default()
            );
            window.lock().unwrap().emit("extract_message", line)?;
            let file_data = fs::read(dir_entry.path())?;
            if let Ok(ninres) = file_data.as_ninres() {
                bundle_ninres(&ninres, &builder, ninres_dir.clone(), mtime)?;
            }
            let c = *completed.read().unwrap() + 1;
            *completed.write().unwrap() = c;
            *progress.write().unwrap() = start_progress + (c as f64 / max_completed as f64) * 2.;
            let extract_progress = (*progress.read().unwrap() as f64 / max_progress as f64) * 100.;
            window
                .lock()
                .unwrap()
                .emit("extract_progress", extract_progress)?;
            Ok(())
        })
        .map(Result::ok)
        .collect::<Vec<_>>();

    let mut builder = builder.into_inner().unwrap();
    builder.finish()?;
    let data = builder.into_inner()?.into_inner();

    increase_progress_sync(window.clone(), progress, max_progress)?;
    window
        .lock()
        .unwrap()
        .emit("extract_step", &format!("{}\nFinished!", file_message))?;
    window.lock().unwrap().emit("extract_message", "Finished")?;

    Ok(data)
}
