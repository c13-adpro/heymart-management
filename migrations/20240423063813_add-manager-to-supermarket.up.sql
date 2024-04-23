-- Add up migration script here
ALTER TABLE supermarket ADD COLUMN
manager_id INT NOT NULL REFERENCES users(id)
ON DELETE CASCADE ON UPDATE CASCADE DEFAULT 1;