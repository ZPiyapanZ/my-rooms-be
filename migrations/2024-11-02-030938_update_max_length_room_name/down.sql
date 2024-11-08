-- This file should undo anything in `up.sql`
ALTER TABLE rooms
ALTER COLUMN room_name TYPE VARCHAR(10);