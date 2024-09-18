-- Add up migration script here

CREATE TABLE short_links (
    id SERIAL PRIMARY KEY,
    short_key VARCHAR(255) NOT NULL UNIQUE,
    url TEXT NOT NULL,
    token VARCHAR(255) NOT NULL UNIQUE,
    clicks INTEGER NOT NULL DEFAULT 0
);
