CREATE TABLE tracks (
    isrc VARCHAR(12) PRIMARY KEY,
    title TEXT NOT NULL,
--     cover_url TEXT,
    duration_ms INTEGER NOT NULL,

    last_fetched TIMESTAMP DEFAULT now() NOT NULL,

--     music services
    spotify_id VARCHAR(22) UNIQUE,
    tidal_id VARCHAR(11) UNIQUE
);
