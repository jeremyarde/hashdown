-- Add migration script here
CREATE TABLE surveys (
    id TEXT PRIMARY KEY,
    plaintext TEXT NOT NULL,
    user_id TEXT,
    created_at TEXT,
    modified_at TEXT version TEXT,
    version TEXT,
    parse_version TEXT
);
CREATE TABLE users (
    id TEXT NOT NULL UNIQUE,
    firstname TEXT,
    lastname TEXT,
    email TEXT NOT NULL,
    created_at TEXT NOT NULL,
    modified_at TEXT NOT NULL
);
Create table answers (
    id TEXT NOT NULL UNIQUE,
    survey_id TEXT NOT NULL,
    survey_version TEXT,
    start_time TEXT,
    end_time TEXT,
    answers TEXT
)