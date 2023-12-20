use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Track {
    pub isrc: String, // id
    pub title: String,
    pub duration_ms: u32,
}
