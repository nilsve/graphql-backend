-- Your SQL goes here
CREATE TABLE notes (
   id SERIAL PRIMARY KEY,
   title VARCHAR(255) NOT NULL,
   body VARCHAR NOT NULL,
   created_at TIMESTAMP DEFAULT NOW()
)
