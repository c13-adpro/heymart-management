-- Add down migration script here
ALTER TABLE supermarket
ALTER COLUMN balance DROP NOT NULL;