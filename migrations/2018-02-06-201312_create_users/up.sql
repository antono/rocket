-- Your SQL goes here
CREATE TABLE Users (
    user_id SERIAL PRIMARY KEY,
    email VARCHAR(100) NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP 
)