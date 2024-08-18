use lettre::{message::header::ContentType, Message, SmtpTransport, Transport};

#[derive(Clone)]
pub struct Mailer {
    mailer: SmtpTransport,
    pub test_from: String,
    smtp_server_url: String,
    pub test_to: String,
}

impl std::fmt::Debug for Mailer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mailer").finish()
    }
}

pub struct EmailIdentity {
    display_name: String,
    email_address: String,
}
impl EmailIdentity {
    fn to_string(&self) -> String {
        format!("{} <{}>", self.display_name, self.email_address)
    }

    pub fn new(display_name: &str, email_address: &str) -> EmailIdentity {
        EmailIdentity {
            display_name: display_name.to_string(),
            email_address: email_address.to_string(),
        }
    }
}

impl Mailer {
    pub fn new(smpt_username: String, smpt_password: String) -> Self {
        use lettre::transport::smtp::authentication::Credentials;

        let from_email: &str = "Test FROM <test@jeremyarde.com>";
        let to_email = "Test TO <test@jeremyarde.com>";
        let smtp_server = "email-smtp.us-east-1.amazonaws.com";

        // let _email = Message::builder()
        //     .from(from_email.parse().unwrap())
        //     // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        //     .to(to_email.parse().unwrap())
        //     .subject("Test email")
        //     .header(ContentType::TEXT_PLAIN)
        //     .body(String::from("Be happy!"))
        //     .unwrap();

        let creds = Credentials::new(
            // dotenvy::var("SMTP_USERNAME").expect("smtp username should be set"),
            // dotenvy::var("SMTP_PASSWORD").expect("smtp password should be set"),
            smpt_username,
            smpt_password,
        );

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(smtp_server)
            .unwrap()
            .credentials(creds)
            // .tls(Tls::Wrapper(TlsParameters::builder(domain)))
            .build();

        Mailer {
            mailer,
            test_from: from_email.to_string(),
            smtp_server_url: smtp_server.to_string(),
            test_to: to_email.to_string(),
        }
    }

    pub fn send(&self, to: EmailIdentity, from: EmailIdentity, message: &str, subject: &str) {
        let email = Message::builder()
            .from(from.to_string().parse().unwrap())
            .to(to.to_string().parse().unwrap())
            .subject(subject.to_string())
            .header(ContentType::TEXT_PLAIN)
            .body(message.to_string())
            .unwrap();

        let _response = &self.mailer.send(&email).expect("Email sent successfully");
    }
}

#[cfg(test)]
mod tests {
    use lettre::transport::smtp;

    use crate::constants::LOGIN_EMAIL_SENDER;

    use super::Mailer;

    #[test]
    fn test_email_send() {
        let smtp_username = dotenvy::var("SMTP_USERNAME").expect("SMTP username not set");
        let smtp_password = dotenvy::var("SMTP_PASSWORD").expect("SMTP password not set");

        let mailer = Mailer::new(smtp_username, smtp_password);
        mailer.send(
            super::EmailIdentity::new("test", "test@jeremyarde.com"),
            super::EmailIdentity::new("Email confirmation", LOGIN_EMAIL_SENDER),
            "Yo this better work",
            "Email confirmation",
        )
    }
}
