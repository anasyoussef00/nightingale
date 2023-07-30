use id3::{Tag, TagLike};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Write},
};
use tauri::api::path::local_data_dir;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub cover: PathBuf,
    pub liked: bool,
    pub path: PathBuf,
}

impl Default for Song {
    fn default() -> Self {
        Self {
            title: String::from("Unknown"),
            artist: String::from("Unknown"),
            album: String::from("Unknown"),
            cover: PathBuf::from("https://placehold.co/300"),
            liked: false,
            path: PathBuf::new(),
        }
    }
}

impl Song {
    pub fn new(
        title: String,
        artist: String,
        album: String,
        cover: PathBuf,
        liked: bool,
        path: PathBuf,
    ) -> Self {
        Self {
            title,
            artist,
            album,
            cover,
            liked,
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
        let data = Tag::read_from_path(&self.path)?;

        let default_song = Self::default();

        let title = match data.title() {
            Some(title) => title.to_string(),
            None => default_song.title,
        };

        let album = match data.album() {
            Some(album) => album.to_string(),
            None => default_song.album,
        };

        let artist = match data.artist() {
            Some(artist) => artist.to_string(),
            None => default_song.artist,
        };

        let path = &self.path;

        Ok(Self {
            title,
            artist,
            album,
            path: path.to_path_buf(),
            ..Default::default()
        })
    }

    pub fn find_song(self) -> Option<Self> {
        let songs_data_path = local_data_dir().unwrap().join("/nightingale/songs.json");
        match self.read_from_local_data_file(&songs_data_path) {
            Ok(songs) => songs.into_iter().find(|song| song.path == self.path),
            Err(..) => None,
        }
    }

    pub fn toggle_song_like(mut self) {
        self.liked = !self.liked
    }
}

pub fn write_to_local_data_file(songs: Vec<Song>, data_file_path: PathBuf) {
    let file = File::create(data_file_path).unwrap();
    let mut writer = BufWriter::new(file);

    serde_json::to_writer_pretty(&mut writer, &songs).unwrap();

    writer.flush().unwrap();
}
