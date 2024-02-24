use std::fmt::Display;

pub const SESSION_ID_KEY: &str = "session_id";
pub const LOGIN_EMAIL_SENDER: &str = "login@gethashdown.com";

pub enum SessionState {
    DELETED,
    ACTIVE,
}

impl Display for SessionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionState::DELETED => write!(f, "DELETED"),
            SessionState::ACTIVE => write!(f, "ACTIVE"),
        }
    }
}
