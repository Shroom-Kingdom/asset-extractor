use crate::{increase_progress, Result};
use std::{
    fs::{read_dir, DirEntry},
    path::Path,
    sync::{Arc, RwLock},
};
use tauri::{
    api::process::{Command, CommandEvent, TerminatedPayload},
    async_runtime, Window,
};
use tempfile::TempDir;

#[allow(clippy::too_many_arguments)]
pub async fn extract_xci(
    window: Window,
    dir: &TempDir,
    romfs_dir: &Path,
    exefs_dir: &Path,
    file: &Path,
    progress: Arc<RwLock<f64>>,
    max_progress: u32,
    file_message: &str,
) -> Result<()> {
    let (mut rx_sidecar, _) = Command::new_sidecar("hactool")?
        .args(vec![
            "--intype=xci",
            "-k",
            "/home/marior/hactool/prod.keys",
            &format!("--securedir={}", dir.path().to_string_lossy()),
            &file.to_string_lossy(),
        ])
        .spawn()?;

    let (tx, mut rx) = async_runtime::channel(1);
    {
        let window = window.clone();
        let progress = progress.clone();
        let file_message = file_message.to_string();
        async_runtime::spawn(async move {
            while let Some(event) = rx_sidecar.recv().await {
                if let CommandEvent::Stdout(line) = event {
                    window.emit("extract_message", line).unwrap();
                }
            }
            increase_progress(window.clone(), progress.clone(), max_progress).unwrap();
            window
                .emit(
                    "extract_step",
                    &format!("{}\nExtracting bundled NCAs", file_message),
                )
                .unwrap();
            tx.send(CommandEvent::Terminated(TerminatedPayload {
                code: Some(0),
                signal: None,
            }))
            .await
            .unwrap();
        });
    }
    rx.recv().await;

    for dir_entry in read_dir(dir.path())? {
        let dir_entry = dir_entry?;
        extract_nca(window.clone(), dir.path(), &dir_entry, romfs_dir, exefs_dir).await?;
    }
    increase_progress(window.clone(), progress.clone(), max_progress)?;

    Ok(())
}

async fn extract_nca(
    window: Window,
    dir: &Path,
    dir_entry: &DirEntry,
    romfs_dir: &Path,
    exefs_dir: &Path,
) -> Result<()> {
    let (mut rx_sidecar, _) = Command::new_sidecar("hactool")
        .expect("failed to create `hactool` binary command")
        .args(vec![
            "-x",
            "-k",
            "/home/marior/hactool/prod.keys",
            &format!("--romfsdir={}", romfs_dir.to_string_lossy()),
            &format!("--exefsdir={}", exefs_dir.to_string_lossy()),
            &dir.to_path_buf()
                .join(dir_entry.file_name())
                .to_string_lossy(),
        ])
        .spawn()
        .expect("Failed to spawn hactool");

    let (tx, mut rx) = async_runtime::channel(1);
    {
        let window = window.clone();
        async_runtime::spawn(async move {
            while let Some(event) = rx_sidecar.recv().await {
                if let CommandEvent::Stdout(line) = event {
                    window
                        .emit("extract_message", line)
                        .expect("failed to emit event");
                }
            }
            tx.send(CommandEvent::Terminated(TerminatedPayload {
                code: Some(0),
                signal: None,
            }))
            .await
            .unwrap();
        });
    }
    rx.recv().await;

    Ok(())
}
