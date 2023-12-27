CREATE TABLE artists (
    id VARCHAR(24) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    image_url TEXT,

    last_fetched TIMESTAMP DEFAULT now() NOT NULL
);
