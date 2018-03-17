-- Create posts table
CREATE TABLE IF NOT EXISTS posts (
  id int(11) NOT NULL AUTO_INCREMENT,
  author_id int(11),
  title varchar(255) NOT NULL UNIQUE,
  body text NOT NULL,
  url_key varchar(50) NOT NULL UNIQUE,
  published boolean NOT NULL DEFAULT 0,
  published_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY(id),
  FOREIGN KEY (author_id) REFERENCES authors(id) ON DELETE CASCADE
)