#[derive(Debug)]
pub struct Song {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

impl Song {
    pub fn new(title: Option<String>, artist: Option<String>, album: Option<String>) -> Self {
        Self {
            title,
            artist,
            album,
        }
    }
}
