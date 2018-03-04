-- Create categories table
CREATE TABLE IF NOT EXISTS categories (
  id integer PRIMARY KEY,
  name text NOT NULL UNIQUE,
  description TEXT NOT NULL,
  url_key TEXT NOT NULL,
  created_at DATE DEFAULT (current_timestamp)
)