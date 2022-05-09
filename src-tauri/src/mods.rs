use crate::Result;
use std::{
    fs::{create_dir_all, File},
    io,
    path::{Path, PathBuf},
};
use tauri::{
    api::process::{Command, CommandEvent, TerminatedPayload},
    async_runtime, Window,
};
use tempfile::TempDir;

pub fn extract_zip(dir: &TempDir, file: &PathBuf) {
    let file = File::open(file).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();
    let temp_dir = dir.path().to_path_buf();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let mut outpath = temp_dir.clone();
        match file.enclosed_name() {
            Some(path) => outpath.push(path),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        #[cfg(unix)]
        {
            use std::{fs, os::unix::fs::PermissionsExt};

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}

pub async fn extract_7z(window: Window, dir: &TempDir, file: &Path) -> Result<()> {
    let (mut rx_sidecar, _) = Command::new_sidecar("7z-sk")
        .expect("failed to create `7z` binary command")
        .args(vec![
            "x",
            "-y",
            "-bd",
            &format!("-o{}", dir.path().to_string_lossy()),
            &file.to_string_lossy(),
        ])
        .spawn()
        .expect("Failed to spawn 7zip");

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
