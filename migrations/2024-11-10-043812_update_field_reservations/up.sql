-- Your SQL goes here
ALTER TABLE reservations
ALTER COLUMN confirmed_at TYPE TIMESTAMPTZ,
ALTER COLUMN cancelled_at TYPE TIMESTAMPTZ;