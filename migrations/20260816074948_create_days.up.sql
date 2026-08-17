CREATE TABLE days (
    id              SERIAL PRIMARY KEY,
    user_id         INTEGER     NOT NULL,
    date            DATE        NOT NULL,
    weight_kg       REAL        NOT NULL,
    target_calories INTEGER     NOT NULL,
    note            TEXT        NOT NULL DEFAULT '',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),

    -- Un solo giorno per utente. È anche il target della foreign key composta
    -- di `entries` (Postgres richiede un indice unico sulle colonne referenziate)
    -- e l'indice usato per le query "i giorni di un utente".
    CONSTRAINT days_user_id_date_key UNIQUE (user_id, date),
    CONSTRAINT days_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES users (id) ON DELETE CASCADE,
    CONSTRAINT days_weight_kg_check       CHECK (weight_kg > 0),
    CONSTRAINT days_target_calories_check CHECK (target_calories >= 0)
);

CREATE TRIGGER days_set_updated_at
    BEFORE UPDATE ON days
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();
