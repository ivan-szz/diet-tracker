CREATE TABLE entries (
    id         SERIAL PRIMARY KEY,
    user_id    INTEGER     NOT NULL,
    date       DATE        NOT NULL,
    name       TEXT        NOT NULL,
    calories   INTEGER     NOT NULL,
    notes      TEXT        NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT entries_day_fkey FOREIGN KEY (user_id, date)
        REFERENCES days (user_id, date) ON DELETE CASCADE,
    CONSTRAINT entries_calories_check CHECK (calories >= 0)
);

-- Postgres non crea indici automatici sulle foreign key: serve per il lookup
-- "tutte le entry di un giorno" e per rendere veloce il CASCADE da `days`.
CREATE INDEX entries_user_id_date_idx ON entries (user_id, date);

CREATE TRIGGER entries_set_updated_at
    BEFORE UPDATE ON entries
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();
