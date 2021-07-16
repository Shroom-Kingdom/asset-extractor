use crate::Result;
use ninres::NinRes;
use std::{
    fs::{self, read_dir, DirEntry},
    path::Path,
    sync::{Arc, RwLock},
};
use tauri::{
    api::process::{Command, CommandEvent, TerminatedPayload},
    async_runtime, Window,
};
use tempfile::tempdir;

pub async fn extract_xci(
    window: Window,
    file: &Path,
    progress: Arc<RwLock<u32>>,
    max_progress: u32,
    file_message: &str,
) -> Result<()> {
    let dir = tempdir()?;
    let romfs_dir = dir.path().join("romfs");
    let exefs_dir = dir.path().join("exefs");

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
            increase_progress(
                &format!("{}\nExtracting bundled NCAs", file_message),
                window,
                progress.clone(),
                max_progress,
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
        extract_nca(
            window.clone(),
            dir.path(),
            &dir_entry,
            &romfs_dir,
            &exefs_dir,
        )
        .await?;
    }
    increase_progress(
        &format!("{}\nExtracting bundled assets", file_message),
        window.clone(),
        progress.clone(),
        max_progress,
    )?;

    let model_dir = romfs_dir.join("Model");
    for dir_entry in read_dir(model_dir)? {
        let dir_entry = dir_entry?;
        dbg!(&dir_entry);
        let file_data = fs::read(dir_entry.path())?;
        if let Ok(ninres) = file_data.as_ninres() {
            dbg!(&ninres);
        }
    }
    increase_progress(
        &format!("{}\nFinished!", file_message),
        window,
        progress.clone(),
        max_progress,
    )?;

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

fn increase_progress(
    step_message: &str,
    window: Window,
    progress: Arc<RwLock<u32>>,
    max_progress: u32,
) -> Result<()> {
    let p = *progress.read().unwrap() + 1;
    *progress.write().unwrap() = p;
    let extract_progress = (p as f64 / max_progress as f64) * 100.;
    window.emit("extract_progress", extract_progress)?;
    window.emit("extract_step", step_message)?;
    Ok(())
}
