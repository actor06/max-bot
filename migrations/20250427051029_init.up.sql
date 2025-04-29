-- migrations/YYYYMMDDHHMMSS_init.sql
CREATE TABLE users (
    user_id BIGINT PRIMARY KEY,
    current_cycle INT NOT NULL DEFAULT 1,
    last_interaction TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE cycles (
    cycle_id SERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(user_id),
    phase1_response TEXT,
    phase2_response TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
