use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Track {
    pub name: String,
    pub duration_ms: i32,
    pub external_ids: TrackExternalIds,

    pub artists: Vec<Artist>
}

#[derive(Deserialize, Serialize)]
pub struct TrackExternalIds {
    pub isrc: String,
}


#[derive(Deserialize, Serialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
}


#[derive(Deserialize, Serialize)]
pub struct Album {
    pub id: String,
    pub name: String,
}
