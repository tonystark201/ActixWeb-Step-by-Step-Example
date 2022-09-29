-- Your SQL goes here

CREATE TABLE teachers (
  uid SERIAL PRIMARY KEY,
  name VARCHAR(512) NOT NULL,
  age INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);