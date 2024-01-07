-- Add migration script here
create schema if not exists mdp;

create table mdp.workspaces (
    id serial primary key,
    workspace_id text not null unique,
    name VARCHAR(255) NOT NULL
);

CREATE table mdp.users (
    id SERIAL,
    user_id TEXT not null unique,
    workspace_id text not null,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    email_status TEXT NOT NULL default 'unverified',

    primary key (workspace_id, user_id),
    foreign key (workspace_id) references mdp.workspaces(workspace_id)
    -- unique (workspace_id, email)
);

CREATE table mdp.surveys (
    id SERIAL,
    survey_id TEXT not null unique,
    workspace_id text not null,
    user_id text not null,
    name VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    plaintext TEXT not null,
    version TEXT,
    parse_version TEXT,
    blocks JSON not null,

    primary key (workspace_id, id),
    foreign key (workspace_id) references mdp.workspaces(workspace_id),
    foreign key(user_id) references mdp.users(user_id)
);

CREATE table mdp.responses (
    id SERIAL,
    workspace_id text not null,
    submitted_at TIMESTAMP WITH TIME ZONE,
    answers JSON,
    survey_id TEXT NOT NULL,

    primary key (workspace_id, id),
    foreign key (workspace_id) references mdp.workspaces(workspace_id),
    foreign key(survey_id) references mdp.surveys(survey_id)
);

-- create table mdp.pageviews (
--     id serial,
--     workspace_id text not null,
--     page_url TEXT not null,
--     created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
--     user_id text,
--     device text,
--     referrer text,
--     ip_address TEXT,
--     user_agent TEXT,

--     primary key (workspace_id, id),
--     foreign key (workspace_id) references mdp.workspaces(workspace_id),
--     foreign key (user_id) references mdp.users(user_id)
-- );

create table mdp.sessions (
    id serial,
    workspace_id text not null,
    session_id TEXT not null unique,
    user_id TEXT NOT NULl unique,
    active_period_expires_at TIMESTAMP with time ZONE DEFAULT CURRENT_TIMESTAMP not null,
    idle_period_expires_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP not null,

    primary key (workspace_id, id),
    foreign key (workspace_id) references mdp.workspaces(workspace_id),
    foreign key(user_id) references mdp.users(user_id)
);

INSERT INTO mdp.workspaces (
    workspace_id,
    name
) VALUES (
    'ws_default',
    'default'
);

INSERT INTO mdp.users (user_id, email, password_hash, created_at, modified_at, email_status, workspace_id) VALUES (
    'usr_default', 
    'test@test.com', 
    '$argon2id$v=19$m=19456,t=2,p=1$JaOOu6OXcVP+B9IUlHX34Q$JGxXSdEtM90s58YlwkIDXn9WfoJTpueOvJrhBlKNF9c', 
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP,
    'verified',
    'ws_default'
), (
    'usr_default2',
    '5jlyqrjzu@mozmail.com',
    '$argon2id$v=19$m=19456,t=2,p=1$ZHAvB4m5UYIZRAteJcLMrQ$l4Lj1wzIBrJ0yK4VyuS5+fMHGXeZyVsIYLuZ9J2UDMA',
    '2023-12-23 18:41:33.418423+00',
    '2023-12-23 18:41:33.418429+00',
    'unverified',
    'ws_default'
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
    workspace_id
) VALUES (
    'k3itjqi4mxhq',	
    'name - todo',
    'usr_default',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP,	
    '# hashdown waitlist :)
Text: Email
textarea: What do you want to use Hashdown for?
Submit: Put me on waitlist',
	'version - todo',
    '2',
    '[{"id":"4cxmez99swdf","index":0,"block_type":"Title","properties":{"type":"Title","title":"Hashdown waitlist :)"}},{"id":"4wgpbx5nqiav","index":0,"block_type":"TextInput","properties":{"type":"TextInput","default":"","question":"Email"}},{"id":"93241ezrlet1","index":0,"block_type":"Textarea","properties":{"type":"Textarea","default":"","question":"What do you want to use Hashdown for?"}},{"id":"svjimprwun33","index":0,"block_type":"Submit","properties":{"type":"Submit","default":"","question":"Put me on waitlist"}},{"id":"3gvtzvmsz1ip","index":0,"block_type":"Empty","properties":{"type":"Nothing"}}]',
    'ws_default'
);
