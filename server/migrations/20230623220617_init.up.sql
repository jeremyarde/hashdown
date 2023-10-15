-- Add migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    user_id TEXT not null unique,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE,
    modified_at TIMESTAMP WITH TIME ZONE,
    deleted_at TIMESTAMP WITH TIME ZONE,
    verified BOOLEAN DEFAULT false
);

CREATE TABLE surveys (
    id SERIAL PRIMARY KEY,
    survey_id TEXT not null unique,
    user_id TEXT,
    created_at TIMESTAMP WITH TIME ZONE,
    modified_at TIMESTAMP WITH TIME ZONE,
    plaintext TEXT,
    version TEXT,
    parse_version TEXT
    
    -- foreign key(user_id) references users(user_id)
);

create table questions (
    id serial primary key, 
    survey_id text not null,
    question_id text not null unique,
    question_type text,

    foreign key(survey_id) references surveys(survey_id)
);

CREATE TABLE responses (
    id SERIAL PRIMARY KEY,
    submitted_at TIMESTAMP WITH TIME ZONE,
    answers JSON,
    survey_id TEXT NOT NULL,

    foreign key(survey_id) references surveys(survey_id)
);

create table pageviews (
    id serial primary key,
    page_url TEXT not null,
    created_at TIMESTAMP WITH TIME ZONE,
    user_id text,
    device text,
    referrer text,
    ip_address TEXT,
    user_agent TEXT
);

INSERT INTO users (user_id, email, password_hash) VALUES (
    'testuserid', 'fake', ''
);

INSERT INTO surveys (survey_id, plaintext, user_id, created_at)
VALUES (
        'testsurveyid', 
        '- q1 title\n  - q1 first question\n  - q1 second\n - q2 question\n  - q2 possible answer',
        'testuserid',
        "2023-09-17 02:50:27.567946+00"
    );
    create table sesssions (
        id serial primary key,
        session_token TEXT,
        user_id TEXT NOT NULl,

        foreign key(user_id) references users(user_id)
    )

