-- Add down migration script here

-- Drop the index (if it exists)
DROP INDEX IF EXISTS idx_short_key;

-- Drop the table
DROP TABLE IF EXISTS short_links;
