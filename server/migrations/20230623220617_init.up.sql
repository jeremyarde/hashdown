-- Add migration script here
CREATE TABLE surveys (
    id SERIAL PRIMARY KEY,
    plaintext TEXT NOT NULL,
    user_id TEXT,
    created_at TEXT,
    modified_at TEXT,
    version TEXT,
    parse_version TEXT
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    user_id TEXT,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    modified_at TEXT NOT NULL,
    verified BOOLEAN DEFAULT false
);

CREATE TABLE answers (
    id SERIAL PRIMARY KEY,
    answer_id TEXT NOT NULL,
    survey_id TEXT NOT NULL,
    survey_version TEXT,
    start_time TEXT,
    end_time TEXT,
    answers JSON,
    created_at TEXT
);

create table pageviews (
    id serial primary key,
    page_url TEXT not null,
    timestamp timestamp not null default CURRENT_TIMESTAMP,
    user_id integer,
    device text,
    referrer text,
    ip_address TEXT,
    user_agent TEXT
);

INSERT INTO surveys (plaintext, user_id)
VALUES (
        '- q1 title\n  - q1 first question\n  - q1 second\n - q2 question\n  - q2 possible answer',
        'testid'
    );

