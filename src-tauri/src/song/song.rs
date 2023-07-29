use id3::TagLike;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Write},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Song {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub path: PathBuf,
}
impl Song {
    pub fn new(
        title: Option<String>,
        artist: Option<String>,
        album: Option<String>,
        path: PathBuf,
    ) -> Self {
        Self {
            title,
            artist,
            album,
            path,
        }
    }

    pub fn read_from_local_data_file(
        &self,
        data_file_path: &PathBuf,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let file = File::open(data_file_path)?;
        let reader = BufReader::new(file);

        let songs = serde_json::from_reader(reader)?;

        Ok(songs)
    }

    pub fn push_to_local_data_file(&self, data_file_path: PathBuf) -> Result<(), Box<dyn Error>> {
        let songs = match self.read_from_local_data_file(&data_file_path) {
            Ok(mut songs) => {
                songs.push(self.clone());
                songs
            }
            Err(_err) => {
                let mut new_songs_vec = vec![];
                new_songs_vec.push(self.clone());
                new_songs_vec
            }
        };

        let file = File::create(data_file_path)?;
        let mut writer = BufWriter::new(file);

        serde_json::to_writer_pretty(&mut writer, &songs)?;

        writer.flush()?;

        Ok(())
    }

    pub fn get_metadata(&self) -> Result<Self, Box<dyn Error>> {
        let data = id3::v1v2::read_from_path(&self.path)?;

        let title = match data.title() {
            Some(title) => Some(title.to_string()),
            None => Some(String::from("")),
        };

        let album = match data.album() {
            Some(album) => Some(album.to_string()),
            None => Some(String::from("Unknown")),
        };

        let artist = match data.artist() {
            Some(artist) => Some(artist.to_string()),
            None => Some(String::from("Unknown")),
        };

        let path = &self.path;

        Ok(Self {
            title,
            artist,
            album,
            path: path.to_path_buf(),
        })
    }
}
