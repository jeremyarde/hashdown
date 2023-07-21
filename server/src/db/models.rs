use chrono::{DateTime, Utc};
use uuid::Uuid;
// use ormlite::Model;
// use uuid::Uuid;
// use ormlite::types::Uuid;

#[derive(Debug)]
struct Session {
    id: Uuid,
    user_id: Uuid,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    factor_id: Option<Uuid>,
    aal: Option<i16>, // Assuming auth.aal_level is an integer type
    not_after: Option<DateTime<Utc>>,
}

#[derive(Debug)]
struct User {
    instance_id: Option<uuid::Uuid>,
    id: uuid::Uuid,
    aud: Option<String>,
    role: Option<String>,
    email: Option<String>,
    encrypted_password: Option<String>,
    email_confirmed_at: Option<DateTime<Utc>>,
    invited_at: Option<DateTime<Utc>>,
    confirmation_token: Option<String>,
    confirmation_sent_at: Option<DateTime<Utc>>,
    recovery_token: Option<String>,
    recovery_sent_at: Option<DateTime<Utc>>,
    email_change_token_new: Option<String>,
    email_change: Option<String>,
    email_change_sent_at: Option<DateTime<Utc>>,
    last_sign_in_at: Option<DateTime<Utc>>,
    raw_app_meta_data: Option<serde_json::Value>,
    raw_user_meta_data: Option<serde_json::Value>,
    is_super_admin: Option<bool>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    phone: Option<String>,
    phone_confirmed_at: Option<DateTime<Utc>>,
    phone_change: Option<String>,
    phone_change_token: Option<String>,
    phone_change_sent_at: Option<DateTime<Utc>>,
    confirmed_at: Option<DateTime<Utc>>,
    email_change_token_current: Option<String>,
    email_change_confirm_status: Option<i16>,
    banned_until: Option<DateTime<Utc>>,
    reauthentication_token: Option<String>,
    reauthentication_sent_at: Option<DateTime<Utc>>,
    is_sso_user: bool,
    deleted_at: Option<DateTime<Utc>>,
}
