use diesel::{QueryDsl, RunQueryDsl};
use chrono::NaiveDateTime;
use crate::helpers::db;

use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};
use crate::models::spotify::Artist;
use crate::schema::tracks;
use crate::utils::tokens::service::Service;
use crate::utils::tokens::spotify::Spotify;

#[derive(AsChangeset, Queryable, Selectable, Deserialize, Serialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::tracks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Track {
    pub isrc: String,
    pub title: String,
    pub duration_ms: i32,

    pub last_fetched: NaiveDateTime,
    pub album_id: String,

    pub spotify_id: Option<String>,
    pub tidal_id: Option<String>,

    pub artists: Vec<Artist>,
}

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct Tracks {
    pub isrc: String,
    pub title: String,
    pub duration_ms: i32,

    pub last_fetched: NaiveDateTime,
    pub album_id: String,

    pub spotify_id: Option<String>,
    pub tidal_id: Option<String>,
}

impl Tracks {
    pub fn find(isrc: &str) -> Result<Self, Error> {
        let conn = &mut db::connection()?;
        let track = tracks::table.filter(tracks::isrc.eq(isrc)).first(conn)?;
        Ok(track)
    }

    pub async fn fetch(isrc: &str) -> Result<Self, Error> {

        let spotify = Spotify::new().await;

        let track = spotify.get_track_by_isrc(isrc).await;
        let track = Tracks::create(track)?;

        Ok(track)
    }

    pub fn create(track: Track) -> Result<Self, Error> {
        let conn = &mut db::connection()?;
        let track = diesel::insert_into(tracks::table)
            .values(Track::from(track))
            .get_result(conn)?;
        Ok(track)
    }

    pub fn update(isrc: &str, track: Track) -> Result<Self, Error> {
        let conn = &mut db::connection()?;
        let track = diesel::update(tracks::table)
            .filter(tracks::isrc.eq(isrc))
            .set(track)
            .get_result(conn)?;
        Ok(track)
    }

    pub fn delete(isrc: &str) -> Result<usize, Error> {
        let conn = &mut db::connection()?;
        let res = diesel::delete(tracks::table.filter(tracks::isrc.eq(isrc))).execute(conn)?;
        Ok(res)
    }
}

impl Track {
    fn from(track: Track) -> Track {
        Track {
            isrc: track.isrc,
            title: track.title,
            duration_ms: track.duration_ms,

            last_fetched: track.last_fetched,
            album_id: String,

            spotify_id: track.spotify_id,
            tidal_id: track.tidal_id,

            artists: track.artists,
        }
    }
}
