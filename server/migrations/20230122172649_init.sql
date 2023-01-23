-- Add migration script here
CREATE TABLE surveys (
    plaintext INTEGER PRIMARY KEY,
    user_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    modified_at TEXT NOT NULL
);
CREATE TABLE users (
    id TEXT NOT NULL UNIQUE,
    firstname TEXT,
    lastname TEXT,
    email TEXT NOT NULL,
    created_at TEXT NOT NULL,
    modified_at TEXT NOT NULL
);