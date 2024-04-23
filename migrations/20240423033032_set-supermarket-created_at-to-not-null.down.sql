-- Add down migration script here
ALTER TABLE supermarket
ALTER COLUMN created_at DROP NOT NULL;