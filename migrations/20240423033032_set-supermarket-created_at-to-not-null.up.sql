-- Add up migration script here
ALTER TABLE supermarket
ALTER COLUMN created_at SET NOT NULL;