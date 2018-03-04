-- Create posts table
CREATE TABLE IF NOT EXISTS posts (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  url_key TEXT NOT NULL,
  title TEXT NOT NULL UNIQUE,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 0,
  created_at DATE DEFAULT (current_timestamp),
  published_at DATE NOT NULL
)