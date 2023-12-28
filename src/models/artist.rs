use chrono::NaiveDateTime;
use crate::helpers::db;

use diesel::prelude::*;
use diesel::result::Error;
use crate::schema::artists;

pub struct Artist {
    pub id: String,
    pub name: String,

    pub last_fetched: NaiveDateTime,

    pub spotify_id: Option<String>,
    pub tidal_id: Option<String>,
}

pub struct Artists {
    pub id: String,
    pub name: String,

    pub last_fetched: NaiveDateTime,

    pub spotify_id: Option<String>,
    pub tidal_id: Option<String>,
}

impl Artists {
    pub fn find(id: &str) -> Result<Self, Error> {
        let conn = &mut db::connection()?;
        let artist = artists::table.filter(artists::id.eq(id)).first(conn)?;
        Ok(artist)
    }

    // pub async fn fetch()

    pub fn create(artist: Artist) -> Result<self, Error> {
        let conn = &mut db::connection()?;
        let artist = diesel::insert_into(artists::table)
            .values(Artist::from(artist))
            .get_result(conn)?;
        Ok(artist)
    }
}

impl Artist {
    pub fn from(artist: Artist) -> Artist {
        Artist {
            id: artist.id,
            name: artist.name,
            last_fetched: artist.last_fetched,

            spotify_id: artist.spotify_id,
            tidal_id: artist.tidal_id,
        }
    }
}
