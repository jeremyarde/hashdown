-- Add migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    user_id TEXT not null unique,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TEXT,
    modified_at TEXT,
    deleted_at TEXT,
    verified BOOLEAN DEFAULT false
);

CREATE TABLE surveys (
    id SERIAL PRIMARY KEY,
    survey_id TEXT not null unique,
    plaintext TEXT,
    user_id TEXT,
    created_at TEXT,
    modified_at TEXT,
    version TEXT,
    parse_version TEXT,
    
    foreign key(user_id) references users(user_id)
);

CREATE TABLE surveys_submissions (
    id SERIAL PRIMARY KEY,
    submitted_at TEXT,
    answers JSON,
    survey_id TEXT NOT NULL,

    foreign key(survey_id) references surveys(survey_id)
);

create table pageviews (
    id serial primary key,
    page_url TEXT not null,
    timestamp timestamp not null default CURRENT_TIMESTAMP,
    user_id text,
    device text,
    referrer text,
    ip_address TEXT,
    user_agent TEXT
);

INSERT INTO users (user_id, email, password_hash) VALUES (
    'testuserid', 'fake', ''
);

INSERT INTO surveys (survey_id, plaintext, user_id)
VALUES (
        'testsurveyid', '- q1 title\n  - q1 first question\n  - q1 second\n - q2 question\n  - q2 possible answer',
        'testuserid'
    );

