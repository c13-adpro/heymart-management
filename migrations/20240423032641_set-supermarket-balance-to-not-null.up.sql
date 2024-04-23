-- Add up migration script here
ALTER TABLE supermarket
ALTER COLUMN balance SET NOT NULL;