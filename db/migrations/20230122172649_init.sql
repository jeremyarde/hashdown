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
    username TEXT,
    password TEXT,
    email TEXT NOT NULL,
    created_at TEXT NOT NULL,
    modified_at TEXT NOT NULL
);
Create table answers (
    id TEXT NOT NULL UNIQUE,
    answer_id TEXT NOT NULL,
    survey_id TEXT NOT NULL,
    survey_version TEXT,
    start_time TEXT,
    end_time TEXT,
    answers JSON,
    created_at TEXT
);
insert into surveys (id, plaintext, user_id)
values (
        'testid',
        '- q1 title\n  - q1 first question\n  - q1 second\n - q2 question\n  - q2 possible answer',
        'statictestuser'
    )