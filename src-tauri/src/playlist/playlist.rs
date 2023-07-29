use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::PathBuf,
};
use tauri::Window;

use crate::song::song::Song;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playlist {
    pub name: String,
    pub path: PathBuf,
    pub songs: Vec<Option<Song>>,
}

impl Playlist {
    pub fn new(name: String, path: PathBuf, songs: Vec<Option<Song>>) -> Self {
        Self { name, path, songs }
    }

    pub fn read_from_local_data_file(
        &self,
        data_file_path: &PathBuf,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let file = File::open(data_file_path)?;
        let reader = BufReader::new(file);

        let playlists = serde_json::from_reader(reader)?;

        Ok(playlists)
    }

    pub fn push_to_local_data_file(&self, data_file_path: PathBuf) -> Result<(), Box<dyn Error>> {
        let playlists = match self.read_from_local_data_file(&data_file_path) {
            Ok(mut playlists) => {
                playlists.push(self.clone());
                playlists
            }
            Err(_err) => {
                let mut new_playlists_vec = vec![];
                new_playlists_vec.push(self.clone());
                new_playlists_vec
            }
        };

        let file = File::create(data_file_path)?;
        let mut writer = BufWriter::new(file);

        serde_json::to_writer_pretty(&mut writer, &playlists)?;

        writer.flush()?;

        Ok(())
    }
}
