CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ユーザの作品情報するための中間テーブル
CREATE TABLE user_works (
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    work_id INT NOT NULL REFERENCES works(id) ON DELETE CASCADE,
    status VARCHAR(50) NOT NULL CHECK (status IN ('NotStarted', 'InProgress', 'Completed')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, work_id)
);

CREATE INDEX idx_user_works_user_status ON user_works(user_id, status);

DROP INDEX idx_works_status;
ALTER TABLE works DROP COLUMN status;
