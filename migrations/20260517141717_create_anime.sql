CREATE TABLE animes (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    episodes INT NOT NULL,
    media_type VARCHAR(50) NOT NULL,
    genre VARCHAR(100) NOT NULL,
    status VARCHAR(50) NOT NULL,
    added_at DATE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);
