-- Add up migration script here
ALTER TABLE IF EXISTS supermarket ADD COLUMN balance INT DEFAULT 0;