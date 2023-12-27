use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::middlewares::error::ErrorResponse;
use crate::models::track::{Tracks};
use crate::utils::project::AppData;

#[derive(Deserialize, Serialize)]
struct TrackResponse {
    track: Tracks,
}

#[get("/tracks/{isrc}")]
pub async fn get_track(req: HttpRequest, _data: web::Data<AppData>) -> Result<HttpResponse, ErrorResponse> {
    let isrc: &str = req.match_info().query("isrc");

    let track = match Tracks::find(isrc) {
        Ok(track) => track,
        Err(diesel::result::Error::NotFound) => Tracks::fetch(isrc).await.ok().unwrap(),
        Err(e) => return Err(ErrorResponse {
            message: format!("Error while querying: {}", e.to_string()),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    };

    Ok(
        HttpResponse::Ok()
            .json(TrackResponse {
                track,
            })
    )
}

// async fn load_track(db: &Client, isrc: &str, spotify_service: &Box<dyn Service>) -> Result<Tracks, Box<dyn Error>> {
//
//     // if Some(db.query_one("SELECT id FROM tracks WHERE id = $1", &[&isrc])) {
//     //     todo!()
//     // };
//
//     let track = &spotify_service.get_track_by_isrc(isrc).await;
//
//     // let rows = db.query(
//     //     "INSERT INTO tracks (title, duration_ms) VALUES ($1, $2)",
//     //     &[&track.title, &track.duration_ms]
//     // ).await?;
//     let rows = db.query(
//         "SELECT $1::TEST",
//         &[&track:]
//     ).await?;
//
//     let value: &str = rows[0].get(0);
//     assert_eq!(value, "hello world");
//
//     Ok(Tracks {
//         isrc: isrc.to_string(),
//         title: "".to_string(),
//         duration_ms: 300,
//     })
// }
