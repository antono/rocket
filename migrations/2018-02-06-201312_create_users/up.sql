-- Your SQL goes here
CREATE TABLE Users (
    user_id SERIAL PRIMARY KEY,
    email VARCHAR(100) NOT NULL,
    published BOOLEAN NOT NULL
)