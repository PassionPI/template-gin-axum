CREATE TABLE IF NOT EXISTS user_base (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(24) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    password VARCHAR(24) NOT NULL,
    role VARCHAR(24),
    nickname VARCHAR(24)
);

CREATE INDEX IF NOT EXISTS idx_user_base_username ON user_base (username);

CREATE INDEX IF NOT EXISTS idx_user_base_nickname ON user_base (nickname);