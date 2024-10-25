-- Add down migration script here
DROP TABLE IF EXISTS notes;
DROP TRIGGER IF EXISTS update_modified_time ON notes;
DROP FUNCTION IF EXISTS update_modified_column();
