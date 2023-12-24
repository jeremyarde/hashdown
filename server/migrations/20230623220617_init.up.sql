-- Add migration script here
create schema if not exists mdp;

-- CREATE TYPE mdp.email_status AS ENUM ('verified', 'unverified');

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
    email_status TEXT NOT NULL default 'unverified'
);

create table mdp.organizations (
    id serial primary key,
    organization_id text not null unique
);

create table mdp.user_organizations (
    user_id text not null,
    organization_id text not null,

    foreign key(user_id) references mdp.users(user_id),
    foreign key (organization_id) references mdp.organizations(organization_id)
);

CREATE table mdp.surveys (
    id SERIAL PRIMARY KEY,
    survey_id TEXT not null unique,
    name TEXT,
    user_id TEXT not null,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    plaintext TEXT not null,
    version TEXT,
    parse_version TEXT,
    blocks JSON not null,
    organization_id TEXT not null,
    
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

INSERT INTO mdp.users (user_id, email, password_hash, created_at, modified_at, email_status) VALUES 
(
    'testuserid', 
    'test@test.com', 
    '$argon2id$v=19$m=19456,t=2,p=1$JaOOu6OXcVP+B9IUlHX34Q$JGxXSdEtM90s58YlwkIDXn9WfoJTpueOvJrhBlKNF9c', 
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP,
    'verified'
),
(
    '1mr4b3zik7qb',
    '5jlyqrjzu@mozmail.com',
    '$argon2id$v=19$m=19456,t=2,p=1$ZHAvB4m5UYIZRAteJcLMrQ$l4Lj1wzIBrJ0yK4VyuS5+fMHGXeZyVsIYLuZ9J2UDMA',
    '2023-12-23 18:41:33.418423+00',
    '2023-12-23 18:41:33.418429+00',
    'unverified'
);

INSERT INTO mdp.surveys (
    survey_id,
    name,
    user_id,
    created_at,
    modified_at,
    plaintext,
    version,
    parse_version,
    blocks,
    organization_id
) VALUES (
    'k3itjqi4mxhq',	'name - todo',
    'nijh7m7klj83',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP,	
    '# hashdown waitlist :)
Text: Email
textarea: What do you want to use Hashdown for?
Submit: Put me on waitlist',
	'version - todo',
    '2',
    '[{"id":"4cxmez99swdf","index":0,"block_type":"Title","properties":{"type":"Title","title":"Hashdown waitlist :)"}},{"id":"4wgpbx5nqiav","index":0,"block_type":"TextInput","properties":{"type":"TextInput","default":"","question":"Email"}},{"id":"93241ezrlet1","index":0,"block_type":"Textarea","properties":{"type":"Textarea","default":"","question":"What do you want to use Hashdown for?"}},{"id":"svjimprwun33","index":0,"block_type":"Submit","properties":{"type":"Submit","default":"","question":"Put me on waitlist"}},{"id":"3gvtzvmsz1ip","index":0,"block_type":"Empty","properties":{"type":"Nothing"}}]',
    'organization here'
);
