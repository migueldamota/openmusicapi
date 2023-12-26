
diesel::table! {
    tracks (isrc) {
        isrc -> VarChar,
        title -> Text,
        duration_ms -> Integer,

        last_fetched -> Timestamp,

        spotify_id -> Nullable<VarChar>,
        tidal_id -> Nullable<VarChar>,
    }
}

diesel::table! {
    tracks_artists (track_isrc, artist_id) {
        track_isrc -> VarChar,
        artist_id -> VarChar,
    }
}


diesel::table! {
    artists (id) {
        id -> VarChar,
        name -> Text,

        last_fetched -> Timestamp,
    }
}
