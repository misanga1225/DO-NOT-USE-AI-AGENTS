CREATE TABLE works (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    episodes INT,
    media_type VARCHAR(50) NOT NULL CHECK (media_type IN ('Novel', 'Anime', 'Manga', 'Game')),
    status VARCHAR(50) NOT NULL CHECK (status IN ('NotStarted', 'InProgress', 'Completed')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_works_status ON works(status);

CREATE TABLE genres (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE
);

-- 作品のジャンルのための中間テーブル
CREATE TABLE work_genres (
    work_id  INT NOT NULL REFERENCES works(id)  ON DELETE CASCADE,
    genre_id INT NOT NULL REFERENCES genres(id) ON DELETE RESTRICT,
    PRIMARY KEY (work_id, genre_id)
);
