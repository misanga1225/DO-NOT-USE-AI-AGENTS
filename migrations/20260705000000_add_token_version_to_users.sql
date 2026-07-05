-- JWT失効用のバージョン番号
-- ログアウト等でインクリメントし，古いトークンを無効化
ALTER TABLE users ADD COLUMN token_version INTEGER NOT NULL DEFAULT 0;
