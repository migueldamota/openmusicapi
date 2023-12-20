use actix_web::{Responder, web, Result, get};
use serde::{Deserialize, Serialize};
use crate::models::track::Track;

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
}

#[derive(Deserialize, Serialize)]
struct SearchResponse {
    items: Vec<Track>
}

#[get("/search")]
pub async fn search() -> Result<impl Responder> {
    let size = 10;
    let mut tracks: Vec<Track> = Vec::with_capacity(size);

    for _ in 0..size {
        tracks.push(Track{
            isrc: "123".to_string(),
            title: "Hullu".to_string(),
            duration: 50,
        });
    }

    let data = SearchResponse{
        items: tracks,
    };

    Ok(web::Json(data))
}
