CREATE TABLE IF NOT EXISTS note (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES user_base (id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    content TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_note_user_id ON note (user_id);