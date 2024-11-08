-- Your SQL goes here
DROP TABLE IF EXISTS rooms;
CREATE TABLE rooms (
    id SERIAL PRIMARY KEY,
    room_name VARCHAR(10) UNIQUE NOT NULL,
    type_id INT REFERENCES room_types(id),
    capacity INT NOT NULL,
    is_available BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);