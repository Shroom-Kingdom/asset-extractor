use crate::Result;
use std::{collections::HashSet, path::PathBuf};

pub fn find_keys() -> Result<Vec<PathBuf>> {
    let mut found_keys: HashSet<PathBuf> = HashSet::new();
    let yuzu_guesses = ["yuzu", "yuzu-emu"];
    let ryujinx_guesses = ["Ryujinx"];
    if let Some(data_dir) = dirs::data_dir() {
        find_key(
            &mut found_keys,
            data_dir.clone(),
            &yuzu_guesses,
            PathBuf::from("keys"),
        )?;
        find_key(
            &mut found_keys,
            data_dir,
            &ryujinx_guesses,
            PathBuf::from("system"),
        )?;
    }
    if let Some(config_dir) = dirs::config_dir() {
        find_key(
            &mut found_keys,
            config_dir.clone(),
            &yuzu_guesses,
            PathBuf::from("keys"),
        )?;
        find_key(
            &mut found_keys,
            config_dir,
            &ryujinx_guesses,
            PathBuf::from("system"),
        )?;
    }
    if let Some(data_local_dir) = dirs::data_local_dir() {
        find_key(
            &mut found_keys,
            data_local_dir.clone(),
            &yuzu_guesses,
            PathBuf::from("keys"),
        )?;
        find_key(
            &mut found_keys,
            data_local_dir,
            &ryujinx_guesses,
            PathBuf::from("system"),
        )?;
    }
    Ok(found_keys.into_iter().collect())
}

fn find_key(
    found_keys: &mut HashSet<PathBuf>,
    dir: PathBuf,
    guesses: &[&str],
    key_path: PathBuf,
) -> Result<()> {
    for guess in guesses.iter() {
        let current_dir = dir.join(guess);
        if current_dir.exists() && current_dir.join(key_path.as_path()).exists() {
            let key = current_dir.join(key_path.as_path()).join("prod.keys");
            if key.exists() {
                found_keys.insert(key);
            }
        }
    }
    Ok(())
}
