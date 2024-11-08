-- This file should undo anything in `up.sql`
ALTER TABLE room_types
DROP CONSTRAINT unique_type_name;
