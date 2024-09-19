CREATE TABLE IF NOT EXISTS friend_request (
    user_by BIGINT NOT NULL,
    user_to BIGINT NOT NULL,
    message VARCHAR(99),
    status VARCHAR(10) NOT NULL DEFAULT 'pending',
    PRIMARY KEY (user_by, user_to),
    FOREIGN KEY (user_by) REFERENCES user_base (id),
    FOREIGN KEY (user_to) REFERENCES user_base (id)
);