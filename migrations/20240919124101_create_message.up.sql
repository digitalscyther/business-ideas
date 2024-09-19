-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE topic (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  name VARCHAR(100) NOT NULL
);

CREATE TABLE message (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  email VARCHAR(100) NOT NULL,
  text TEXT NOT NULL DEFAULT '',
  topic_id uuid REFERENCES topic(id) NOT NULL
);
