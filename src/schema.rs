// @generated automatically by Diesel CLI.

diesel::table! {
    artists (id) {
        #[max_length = 24]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        image_url -> Nullable<Text>,
        last_fetched -> Timestamp,
    }
}

diesel::table! {
    tracks (isrc) {
        #[max_length = 12]
        isrc -> Varchar,
        title -> Text,
        duration_ms -> Int4,
        last_fetched -> Timestamp,
        #[max_length = 22]
        spotify_id -> Nullable<Varchar>,
        #[max_length = 11]
        tidal_id -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    artists,
    tracks,
);
