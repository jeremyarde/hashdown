-- Add migration script here
create schema if not exists mdp;

CREATE TYPE mdp.email_status AS ENUM ('verified', 'unverified');

CREATE table mdp.magic_links (
    
);

CREATE table mdp.users (
    id SERIAL PRIMARY KEY,
    user_id TEXT not null unique,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    email_status mdp.email_status NOT NULL default 'unverified'
);

CREATE table mdp.surveys (
    id SERIAL PRIMARY KEY,
    survey_id TEXT not null unique,
    name TEXT,
    user_id TEXT not null,
    created_at TIMESTAMP not null WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    modified_at TIMESTAMP not null WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    plaintext TEXT not null,
    version TEXT,
    parse_version TEXT,
    
    foreign key(user_id) references mdp.users(user_id)
);

create table mdp.questions (
    id serial primary key, 
    survey_id text not null,
    question_id text not null unique,
    question_type text,

    foreign key(survey_id) references mdp.surveys(survey_id)
);

CREATE table mdp.responses (
    id SERIAL PRIMARY KEY,
    submitted_at TIMESTAMP WITH TIME ZONE,
    answers JSON,
    survey_id TEXT NOT NULL,

    foreign key(survey_id) references mdp.surveys(survey_id)
);

create table mdp.pageviews (
    id serial primary key,
    page_url TEXT not null,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    user_id text,
    device text,
    referrer text,
    ip_address TEXT,
    user_agent TEXT
);

create table mdp.sessions (
    id serial primary key,
    session_id TEXT not null unique,
    user_id TEXT NOT NULl unique,
    active_period_expires_at TIMESTAMP with time ZONE DEFAULT CURRENT_TIMESTAMP not null,
    idle_period_expires_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP not null,

    foreign key(user_id) references mdp.users(user_id)
);

INSERT INTO mdp.users (user_id, email, password_hash, email_status, created_at, modified_at) VALUES (
    'testuserid', 
    'test@test.com', 
    '$argon2id$v=19$m=19456,t=2,p=1$JaOOu6OXcVP+B9IUlHX34Q$JGxXSdEtM90s58YlwkIDXn9WfoJTpueOvJrhBlKNF9c', 
    'verified',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP

);

INSERT INTO mdp.surveys (survey_id, plaintext, user_id, created_at)
VALUES (
        'testsurveyid', 
        '- q1 title\n  - q1 first question\n  - q1 second\n - q2 question\n  - q2 possible answer',
        'testuserid',
        '2023-09-17 02:50:27.567946+00'
    );
