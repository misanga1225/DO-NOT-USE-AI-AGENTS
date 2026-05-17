-- Add migration script here
CREATE TABLE animes (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    episodes INT NOT NULL,
    created_ad TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);
