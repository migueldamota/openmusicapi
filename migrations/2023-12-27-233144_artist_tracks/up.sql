-- Your SQL goes here
CREATE TABLE artist_tracks (
    artist_id VARCHAR NOT NULL,
    track_id VARCHAR NOT NULL,
    PRIMARY KEY (artist_id, track_id)
);
