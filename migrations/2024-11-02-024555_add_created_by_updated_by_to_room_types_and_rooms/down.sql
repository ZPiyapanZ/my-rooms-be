-- This file should undo anything in `up.sql`
-- room_types
ALTER TABLE room_types
DROP COLUMN IF EXISTS created_by,
DROP COLUMN IF EXISTS updated_by;

-- rooms
ALTER TABLE rooms
DROP COLUMN IF EXISTS created_by,
DROP COLUMN IF EXISTS updated_by;
