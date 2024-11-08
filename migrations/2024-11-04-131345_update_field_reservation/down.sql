-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS reservations;
CREATE TABLE reservations (
    id SERIAL PRIMARY KEY,
    room_id INT REFERENCES rooms(id),
    customer_contact_id INT REFERENCES customer_contacts(id),
    check_in_date DATE NOT NULL,
    check_out_date DATE NOT NULL,
    total_price DECIMAL(10, 2) NOT NULL,
    status VARCHAR(20) DEFAULT 'pending',

    created_by INT REFERENCES staff(id),
    confirmed_by INT REFERENCES staff(id),
    cancelled_by INT REFERENCES staff(id),

    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);