-- Add up migration script here
CREATE TABLE IF NOT EXISTS notes (
    id CHAR(36) PRIMARY KEY NOT NULL,
    title VARCHAR(255) NOT NULL UNIQUE,
    content TEXT NOT NULL,
    is_published BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
NEW.updated_at = now();
RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_modified_time BEFORE UPDATE ON notes FOR EACH ROW EXECUTE PROCEDURE update_modified_column();
