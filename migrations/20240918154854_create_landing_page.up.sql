-- Add up migration script here

CREATE TABLE landing_page (
    id SERIAL PRIMARY KEY,
    path VARCHAR(255) NOT NULL UNIQUE,
    html BYTEA NOT NULL
);
