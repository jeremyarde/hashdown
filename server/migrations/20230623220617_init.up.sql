-- Your SQL goes here
-- Add migration script here
create schema if not exists mdp;

create table mdp.workspaces (
    -- id serial primary key,
    -- id serial,
    workspace_id text not null unique,
    name VARCHAR(255) NOT NULL,

    primary key (workspace_id)
);

CREATE table mdp.users (
    -- id serial,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    email_status TEXT NOT NULL default 'unverified',
    user_id TEXT not null unique,
    workspace_id text not null,
    email_confirmed_at TIMESTAMP with time zone,
    confirmation_token TEXT,
    confirmation_token_expire_at TIMESTAMP WITH TIME ZONE,

    role TEXT,

    -- stripe related stuff
    stripe_customer_id TEXT unique,
    stripe_subscription_id TEXT,
    stripe_subscription_modified_at TIMESTAMP WITH TIME ZONE,
    stripe_subscription_price_id TEXT,
    -- stripe_subscription_id

    -- primary key (user_id),
    -- primary key (workspace_id, id),
    primary key (workspace_id, user_id),
    -- foreign key (workspace_id) references mdp.workspaces(id)
    foreign key (workspace_id) references mdp.workspaces(workspace_id)
);

-- Features table to store the list of features that can be enabled for accounts
-- we want to be able to enable/disable features for each account, at the user/account level
-- we may also care about workspace level features, but that can be a new table
CREATE TABLE mdp.features (
    feature_id TEXT NOT NULL UNIQUE,
    -- feature_id SERIAL PRIMARY KEY,  -- Adding this primary key
    feature_name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    workspace_id text NOT NULL,
    foreign key (workspace_id) references mdp.workspaces(workspace_id)
);

CREATE TABLE mdp.user_features (
    user_id TEXT NOT NULL,
    feature_id TEXT NOT NULL,
    enabled BOOLEAN DEFAULT TRUE,
    enabled_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    workspace_id text NOT NULL,

    foreign key (feature_id) references mdp.features(feature_id),
    foreign key (user_id) references mdp.users(user_id),
    foreign key (workspace_id) references mdp.workspaces(workspace_id),

    PRIMARY KEY (workspace_id, user_id, feature_id)  -- Composite primary key
);

create table mdp.stripe_events (
    -- id serial,
    stripe_event_id text not null unique,
    from_stripe_event_id text not null,
    attributes JSON,
    event_type Text not null,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    workspace_id text not null,

    -- primary key (stripe_event_id)
    primary key (workspace_id, stripe_event_id),
    -- foreign key (workspace_id) references mdp.workspaces(id)
    foreign key (workspace_id) references mdp.workspaces(workspace_id)

);

CREATE table mdp.surveys (
    -- id serial,
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

    -- primary key (workspace_id, id),
    primary key (workspace_id, survey_id),
    -- foreign key (workspace_id) references mdp.workspaces(id),
    foreign key (workspace_id) references mdp.workspaces(workspace_id),
    foreign key(user_id) references mdp.users(user_id)
);

CREATE table mdp.responses (
    -- id serial,
    response_id TEXT not null unique,
    workspace_id text not null,
    submitted_at TIMESTAMP WITH TIME ZONE,
    answers JSON,
    survey_id TEXT NOT NULL,

    -- primary key (response_id),
    -- primary key (workspace_id, id),
    primary key (workspace_id, response_id),
    foreign key (workspace_id) references mdp.workspaces(workspace_id),
    foreign key (survey_id) references mdp.surveys(survey_id)
);


CREATE TABLE mdp.sessions (
    session_id TEXT NOT NULL UNIQUE,
    workspace_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    active_period_expires_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    idle_period_expires_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    current_state TEXT DEFAULT 'ACTIVE' NOT NULL,
    PRIMARY KEY (workspace_id, session_id),
    foreign key (workspace_id) references mdp.workspaces(workspace_id),
    foreign key (user_id) references mdp.users(user_id) ON DELETE CASCADE  -- Added reference to `user_id`
);

/* Useful to capture current analytics associated with usage of the api */
-- create table mdp.usage ();
-- /* Used to store information about API keys that users have created to allow  */
-- create table mdp.api_keys ();
-- create table mdp.limits ();
-- /* https://chat.openai.com/c/43c833bf-c2a2-4306-9d38-968cba5168cf*/
-- create table mdp.subscriptions (
--     id serial,
--     subscription_id text not null unique,
--     user_id text not null unique,
--     subscription_plan_id text not null,
--     created_at text not null,
--     updated_at text not null,
-- );

/* useful for frontend to show # responses on listsurvey screen */
create materialized view
  mdp.survey_summary as
select
  responses.survey_id,
  count(*) as count
from
  mdp.responses
group by
  responses.survey_id;

INSERT INTO mdp.workspaces (
    workspace_id,
    name
) VALUES (
    'ws_default',
    'default'
), (
    'ws_test',
    'test_workspace'
);


INSERT INTO mdp.users (
    name, user_id, email, password_hash, created_at, modified_at, email_status, workspace_id, confirmation_token
) VALUES (
    'test1',
    'usr_default', 
    'test@test.com', 
    '$argon2id$v=19$m=19456,t=2,p=1$JaOOu6OXcVP+B9IUlHX34Q$JGxXSdEtM90s58YlwkIDXn9WfoJTpueOvJrhBlKNF9c', 
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP,
    'verified',
    'ws_default',
    'cfm_fake'
), (
    'test2',
    'usr_default2',
    '5jlyqrjzu@mozmail.com',
    '$argon2id$v=19$m=19456,t=2,p=1$ZHAvB4m5UYIZRAteJcLMrQ$l4Lj1wzIBrJ0yK4VyuS5+fMHGXeZyVsIYLuZ9J2UDMA',
    '2023-12-23 18:41:33.418423+00',
    '2023-12-23 18:41:33.418429+00',
    'unverified',
    'ws_default',
    'cfm_fake2'
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
    '# Get emailed when hashdown is available
Text: Email
textarea: What do you want to use Hashdown for?
Submit: Put me on waitlist',
	'version - todo',
    '2',
    '[{"id":"4cxmez99swdf","index":0,"block_type":"Title","properties":{"type":"Title","title":"Get emailed when hashdown is available"}},{"id":"4wgpbx5nqiav","index":0,"block_type":"TextInput","properties":{"type":"TextInput","default":"","question":"Email"}},{"id":"93241ezrlet1","index":0,"block_type":"Textarea","properties":{"type":"Textarea","default":"","question":"What do you want to use Hashdown for?"}},{"id":"svjimprwun33","index":0,"block_type":"Submit","properties":{"type":"Submit","default":"Submit","button":"Put me on waitlist"}},{"id":"3gvtzvmsz1ip","index":0,"block_type":"Empty","properties":{"type":"Nothing"}}]',
    'ws_default'
);
