use actix_web::{Responder, Result, get, web, HttpRequest};
use serde::{Deserialize, Serialize};
use crate::models::track::Track;
use crate::utils::project::AppData;
use crate::utils::tokens::service::{Services};

#[derive(Deserialize, Serialize)]
struct SearchResponse {
    items: Track,
}

#[get("/tracks/{isrc}")]
pub async fn get_track(req: HttpRequest, data: web::Data<AppData>) -> Result<impl Responder> {
    let isrc: &str = req.match_info().query("isrc");

    let services = data.services.lock().map_err(|_| {
        actix_web::error::ErrorInternalServerError("Failed to lock services mutex")
    })?;
    let spotify_service = services.get(&Services::Spotify).ok_or_else(|| {
        actix_web::error::ErrorInternalServerError("Spotify service not available")
    })?;

    let track = spotify_service.get_track_by_isrc(isrc).await;

    let data = SearchResponse{
        items: track,
    };

    Ok(web::Json(data))
}
