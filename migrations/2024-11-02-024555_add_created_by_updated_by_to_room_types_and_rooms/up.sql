-- Your SQL goes here
-- room_types
ALTER TABLE room_types
ADD COLUMN created_by INT REFERENCES staff(id),
ADD COLUMN updated_by INT REFERENCES staff(id);

-- rooms
ALTER TABLE rooms
ADD COLUMN created_by INT REFERENCES staff(id),
ADD COLUMN updated_by INT REFERENCES staff(id);
