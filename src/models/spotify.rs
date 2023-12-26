use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Track {
    pub name: String,
    pub duration_ms: i32,
    pub external_ids: TrackExternalIds,
}

#[derive(Deserialize, Serialize)]
pub struct TrackExternalIds {
    pub isrc: String,
}
