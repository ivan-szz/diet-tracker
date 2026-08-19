ALTER TABLE entries
    DROP CONSTRAINT entries_day_fkey,
    ADD CONSTRAINT entries_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES users (id) ON DELETE CASCADE;
