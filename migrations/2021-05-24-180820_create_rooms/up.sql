-- Your SQL goes here

CREATE TABLE ROOMS (
  id SERIAL PRIMARY KEY,
  user_1_id INTEGER REFERENCES USERS (id) NOT NULL,
  user_2_id INTEGER REFERENCES USERS (id) NOT NULL
)