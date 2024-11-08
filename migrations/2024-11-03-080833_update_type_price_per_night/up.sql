-- Your SQL goes here
ALTER TABLE room_types
ALTER COLUMN price_per_night SET DATA TYPE INTEGER,
ALTER COLUMN price_per_night SET NOT NULL;