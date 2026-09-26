ALTER TABLE users
    ADD COLUMN streak INTEGER NOT NULL DEFAULT 0,
    ADD CONSTRAINT users_streak_check CHECK (streak >= 0);

ALTER TABLE days
    DROP COLUMN weight_logged_at;
