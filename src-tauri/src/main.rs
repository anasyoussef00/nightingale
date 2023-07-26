// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod metadata;
mod song;

fn main() {
    let song = metadata::metadata::read_song_metadata("src/mp3/kerosene.mp3").expect("CAPEGPOSG");
    println!("{:#?}", song);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
