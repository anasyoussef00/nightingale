// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::File, io::BufReader, path::Path};

use menu::build_menu;
use playlist::playlist::Playlist;
use tauri::{
    api::{
        dialog,
        dir::read_dir,
        path::{audio_dir, data_dir, local_data_dir},
    },
    Window,
};

use crate::song::song::Song;

mod menu;
mod playlist;
mod song;

#[tauri::command]
fn playlist_selected(window: Window, playlist: Playlist) {
    window.emit("playlist-selected", playlist).unwrap();
}

#[tauri::command]
fn song_selected(window: Window, song: Song) {
    window.emit("song-selected", song).unwrap();
}

#[tauri::command]
fn fetch_added_songs() -> Vec<Song> {
    let formatted_path = format!(
        "{}/nightingale/songs.json",
        local_data_dir().unwrap().display()
    );
    let path = Path::new(&formatted_path);

    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);

            let songs: Vec<Song> = serde_json::from_reader(reader).unwrap();

            songs
        }
        Err(_) => [].to_vec(),
    }
}

#[tauri::command]
fn toggle_song_like(song: Song) {
    let found_song = song.clone().find_song().unwrap();
    found_song.clone().toggle_song_like();

    let data_file_path = data_dir().unwrap().join("/nightingale/songs.json");
    let mut songs = song.read_from_local_data_file(&data_file_path).unwrap();
    songs.retain(|s| s.path == song.path);
    songs.push(found_song);
    song::song::write_to_local_data_file(songs, data_file_path)
}

fn main() {
    let menu = build_menu();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            playlist_selected,
            song_selected,
            fetch_added_songs,
            toggle_song_like
        ])
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "open-single-file" => dialog::FileDialogBuilder::default()
                .add_filter("Audio", &["mp3", "ogg", "wav", "flac"])
                .set_directory(audio_dir().unwrap())
                .pick_file(move |path_buf| match path_buf {
                    Some(p) => {
                        if let Some(dir) = local_data_dir() {
                            let song = Song {
                                path: p,
                                ..Default::default()
                            };
                            let song_with_metadata = song.get_metadata().unwrap();
                            let formatted_path =
                                format!("{}/nightingale/songs.json", dir.display());
                            let path = Path::new(&formatted_path);
                            song_with_metadata
                                .push_to_local_data_file(path.to_path_buf())
                                .unwrap();

                            song_selected(event.window().to_owned(), song_with_metadata);
                        }
                    }
                    _ => {}
                }),
            "open-folder" => {
                dialog::FileDialogBuilder::default().pick_folder(move |path_buf| match path_buf {
                    Some(p) => {
                        if let Some(dir) = local_data_dir() {
                            let files = read_dir(&p, true).unwrap();
                            let mut songs: Vec<Option<Song>> = vec![];
                            for _file in files {
                                let formatted_path =
                                    format!("{}/nightingale/songs.json", dir.display());
                                let path = Path::new(&formatted_path);
                                let song = Song {
                                    path: path.to_path_buf(),
                                    ..Default::default()
                                };
                                let song_with_metadata = song.get_metadata().unwrap();

                                songs.push(Some(song_with_metadata));
                            }

                            let playlist = Playlist::new(
                                Path::file_name(&p)
                                    .unwrap()
                                    .to_os_string()
                                    .into_string()
                                    .unwrap(),
                                p,
                                songs,
                            );
                            let formatted_path =
                                format!("{}/nightingale/playlists.json", dir.display());
                            let path = Path::new(&formatted_path);

                            playlist
                                .push_to_local_data_file(path.to_path_buf())
                                .unwrap();

                            playlist_selected(event.window().to_owned(), playlist);
                        }
                    }
                    _ => {}
                })
            }
            "quit" => std::process::exit(0),
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
