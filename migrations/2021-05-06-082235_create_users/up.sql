-- Your SQL goes here

CREATE TABLE USERS (
  id SERIAL PRIMARY KEY,
  login VARCHAR NOT NULL UNIQUE,
  password_hash VARCHAR NOT NULL
)
