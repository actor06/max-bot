CREATE TABLE IF NOT EXISTS users (
    user_id BIGINT PRIMARY KEY,
    current_cycle INT DEFAULT 1,
    last_message TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS cycles (
    cycle_id SERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(user_id),
    phase1_response TEXT,
    phase2_response TEXT
);
