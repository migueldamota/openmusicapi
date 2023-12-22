use actix_web::{Responder, Result, get, web};
use serde::{Deserialize, Serialize};
use crate::models::track::Track;
use crate::utils::project::{AppState};
use crate::utils::tokens::service::{Services};
use crate::utils::tokens::spotify::Spotify;

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
}

#[derive(Deserialize, Serialize)]
struct SearchResponse {
    items: Track
}

#[get("/search")]
pub async fn search() -> Result<impl Responder> {
    let spotify = Spotify::new();

    let track = spotify.get_track_by_isrc("QZXDB2300042").await;

    let data = SearchResponse{
        items: track,
    };

    Ok(web::Json(data))
}
