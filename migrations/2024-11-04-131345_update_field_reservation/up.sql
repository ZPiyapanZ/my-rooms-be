-- Your SQL goes here
DROP TABLE IF EXISTS reservations;
CREATE TABLE reservations (
    id SERIAL PRIMARY KEY,
    room_id INT REFERENCES rooms(id) NOT NULL,
    customer_contact_id INT REFERENCES customer_contacts(id) NOT NULL,
    check_in_date DATE NOT NULL,
    check_out_date DATE NOT NULL,
    total_price INTEGER NOT NULL,
    status VARCHAR(20) DEFAULT 'pending' NOT NULL,

    created_by INT REFERENCES staff(id),
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,

    updated_by INT REFERENCES staff(id),
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,

    confirmed_by INT REFERENCES staff(id),
    confirmed_at TIMESTAMPTZ DEFAULT NOW(),

    cancelled_by INT REFERENCES staff(id),
    cancelled_at TIMESTAMPTZ DEFAULT NOW()
);