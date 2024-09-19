CREATE TABLE IF NOT EXISTS friend (
    id_a BIGINT NOT NULL,
    id_b BIGINT NOT NULL,
    status VARCHAR(10) NOT NULL DEFAULT 'ok',
    PRIMARY KEY (id_a, id_b),
    FOREIGN KEY (id_a) REFERENCES user_base (id),
    FOREIGN KEY (id_b) REFERENCES user_base (id),
    CHECK (id_a < id_b)
);

CREATE INDEX IF NOT EXISTS idx_friend_id_a ON friend (id_a);

CREATE INDEX IF NOT EXISTS idx_friend_id_b ON friend (id_b);

CREATE INDEX IF NOT EXISTS idx_friend_status ON friend (status);