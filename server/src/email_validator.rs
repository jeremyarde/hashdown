use std::collections::HashSet;

use crate::ServerError;

pub fn is_burner_email_provider(email: &str) -> Result<bool, ServerError> {
    let domain = email
        .split("@")
        .nth(1)
        .ok_or(ServerError::RequestParams("Email not accepted".to_string()))?;
    let bad_domains: HashSet<&str> = include_str!("../../server/static/emails.txt")
        .lines()
        .map(|s| s)
        .collect();

    return Ok(bad_domains.contains(domain));
}

#[cfg(test)]
mod tests {
    use crate::email_validator::is_burner_email_provider;

    #[test]
    fn test_bad_domain() {
        let email = "myemail@07-izvestiya.ru";

        assert_eq!(is_burner_email_provider(email).unwrap(), true);
    }

    #[test]
    fn test_good_domain() {
        let email = "myemail@google.com";

        assert_eq!(is_burner_email_provider(email).unwrap(), false);
    }
}
