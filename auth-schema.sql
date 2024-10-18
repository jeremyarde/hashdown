CREATE TABLE organization (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE permission (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE role (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    org_id UUID REFERENCES organization(id) ON DELETE CASCADE
);

CREATE TABLE user (
    id UUID PRIMARY KEY,
    first_name TEXT,
    last_name TEXT,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE user_session (
    id UUID PRIMARY KEY,
    expires_at TIMESTAMPTZ NOT NULL,
    user_id UUID REFERENCES user(id) ON DELETE CASCADE
);

CREATE TABLE user_org_role (
    user_id UUID REFERENCES user(id) ON DELETE CASCADE,
    org_id UUID REFERENCES organization(id) ON DELETE CASCADE,
    role_id UUID REFERENCES role(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (user_id, org_id, role_id)
);

CREATE TABLE role_permission (
    role_id UUID REFERENCES role(id) ON DELETE CASCADE,
    permission_id UUID REFERENCES permission(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (role_id, permission_id)
);
