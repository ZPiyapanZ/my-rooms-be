-- This file should undo anything in `up.sql`
ALTER TABLE customer_contacts
ALTER COLUMN phone_number VARCHAR(15);