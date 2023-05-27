-- Add migration script here
CREATE TABLE cats (
  id BIGSERIAL PRIMARY KEY,
  name VARCHAR (255) UNIQUE NOT NULL
);