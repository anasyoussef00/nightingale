use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::BufReader, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct SongPathJson {
    pub path: PathBuf,
}

impl SongPathJson {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}
