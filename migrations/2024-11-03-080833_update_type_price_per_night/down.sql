-- This file should undo anything in `up.sql`
ALTER TABLE room_types
ALTER COLUMN price_per_night DECIMAL(10, 2) NOT NULL;
