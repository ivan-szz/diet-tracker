CREATE TABLE users (
    id               SERIAL PRIMARY KEY,
    name             TEXT        NOT NULL,
    password_hash    TEXT        NOT NULL,
    streak           INTEGER     NOT NULL DEFAULT 0,
    target_weight_kg REAL        NOT NULL,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT users_name_key         UNIQUE (name),
    CONSTRAINT users_streak_check     CHECK (streak >= 0),
    CONSTRAINT users_target_weight_kg_check CHECK (target_weight_kg > 0)
);

CREATE TRIGGER users_set_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();
