-- When the weight was first logged, so a day only counts towards the streak if
-- something was actually tracked in time (see `server::services::streak`).
ALTER TABLE days
    ADD COLUMN weight_logged_at TIMESTAMPTZ;

UPDATE days
SET weight_logged_at = created_at
WHERE weight_kg IS NOT NULL;

ALTER TABLE users
    DROP COLUMN streak;
