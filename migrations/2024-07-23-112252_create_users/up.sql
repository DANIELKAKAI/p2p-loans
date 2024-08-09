CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR(256) NOT NULL,
  last_name VARCHAR(256) NOT NULL,
  email VARCHAR(256) NOT NULL UNIQUE,
  password VARCHAR(256) NOT NULL,
  active BOOLEAN NOT NULL DEFAULT TRUE,
  updated_at timestamp  without time zone default CURRENT_TIMESTAMP not null,
  created_at timestamp  without time zone default CURRENT_TIMESTAMP not null
);