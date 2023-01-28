-- Add migration script here
CREATE TABLE surveys (
    id TEXT PRIMARY KEY,
    plaintext TEXT NOT NULL,
    user_id TEXT,
    created_at TEXT,
    modified_at TEXT
);
CREATE TABLE users (
    id TEXT NOT NULL UNIQUE,
    firstname TEXT,
    lastname TEXT,
    email TEXT NOT NULL,
    created_at TEXT NOT NULL,
    modified_at TEXT NOT NULL
);