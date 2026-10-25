-- Add up migration script here
CREATE TABLE IF NOT EXISTS tasks(
    id BIGSERIAL PRIMARY KEY,
    title TEXT NOT NULL
);
