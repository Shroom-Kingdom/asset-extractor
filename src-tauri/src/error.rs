use nfd2::error::NfdError;
use serde::Serialize;
use std::io;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum Error {
    #[error("[Nfd]: {}", 0)]
    Nfd(String),
    #[error("[Io]: {}", 0)]
    Io(String),
    #[error("[Tauri]: {}", 0)]
    Tauri(String),
    #[error("[TauriApi]: {}", 0)]
    TauriApi(String),
    #[error("[NinRes]: {}", 0)]
    NinRes(String),
    #[error("File select canceled")]
    FileSelectCanceled,
    #[error("File extension not supported")]
    FileExtensionUnsupported,
}

impl From<NfdError> for Error {
    fn from(err: NfdError) -> Error {
        Error::Nfd(format!("{:?}", err))
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(format!("{:?}", err))
    }
}

impl From<tauri::Error> for Error {
    fn from(err: tauri::Error) -> Error {
        Error::Tauri(format!("{:?}", err))
    }
}

impl From<tauri::api::Error> for Error {
    fn from(err: tauri::api::Error) -> Error {
        Error::TauriApi(format!("{:?}", err))
    }
}

impl From<ninres::NinResError> for Error {
    fn from(err: ninres::NinResError) -> Error {
        Error::NinRes(format!("{:?}", err))
    }
}
