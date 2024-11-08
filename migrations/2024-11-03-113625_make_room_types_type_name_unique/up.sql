-- Your SQL goes here
ALTER TABLE room_types
ADD CONSTRAINT unique_type_name UNIQUE (type_name);