// use std::error::Error;
//
// use id3::{Tag, TagLike};
//
// use crate::song::song::Song;
//
// // #[tauri::command]
// pub fn read_song_metadata(path: &str) -> Result<Song, Box<dyn Error>> {
//     let tag = match Tag::read_from_path(path) {
//         Ok(tag) => tag,
//         Err(err) => return Err(Box::new(err)),
//     };
//
//     let title = if let Some(title) = tag.title() {
//         Some(title.to_string())
//     } else {
//         None
//     };
//
//     let artist = if let Some(artist) = tag.artist() {
//         Some(artist.to_string())
//     } else {
//         None
//     };
//
//     let album = if let Some(album) = tag.album() {
//         Some(album.to_string())
//     } else {
//         None
//     };
//
//     let song = Song::new(title, artist, album);
//
//     Ok(song)
// }
