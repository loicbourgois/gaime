CREATE TABLE IF NOT EXISTS users(
    id SERIAL PRIMARY KEY,
    username VARCHAR(128) UNIQUE NOT NULL,
    email VARCHAR(128) UNIQUE NOT NULL,
    hash VARCHAR(128) NOT NULL,
    created_date TIMESTAMP,
    modified_date TIMESTAMP
);

ALTER TABLE users ALTER COLUMN created_date SET DEFAULT now();
ALTER TABLE users ALTER COLUMN modified_date SET DEFAULT now();
