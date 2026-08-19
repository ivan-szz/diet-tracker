ALTER TABLE entries
    DROP CONSTRAINT entries_user_id_fkey,
    ADD CONSTRAINT entries_day_fkey FOREIGN KEY (user_id, date)
        REFERENCES days (user_id, date) ON DELETE CASCADE;
