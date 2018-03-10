-- Create posts table
CREATE TABLE IF NOT EXISTS posts (
  id int(11) NOT NULL AUTO_INCREMENT,
  title varchar(50) NOT NULL UNIQUE,
  body text NOT NULL,
  url_key varchar(50) NOT NULL UNIQUE,
  published boolean NOT NULL DEFAULT 0,
  published_at date NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
  PRIMARY KEY(id)
)