use lettre::{
    message::{header::ContentType, MessageBuilder},
    Message, SmtpTransport, Transport,
};

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

impl Mailer {
    pub fn new() -> Self {
        use lettre::message::header::ContentType;
        use lettre::transport::smtp::authentication::Credentials;
        use lettre::{Message, SmtpTransport};

        let from_email = "Test FROM <test@jeremyarde.com>";
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
            dotenvy::var("SMTP_USERNAME").expect("smtp username should be set"),
            dotenvy::var("SMTP_PASSWORD").expect("smtp password should be set"),
        );

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(smtp_server)
            .unwrap()
            .credentials(creds)
            // .tls(Tls::Wrapper(TlsParameters::builder(domain)))
            .build();

        Mailer {
            mailer: mailer,
            test_from: from_email.to_string(),
            smtp_server_url: smtp_server.to_string(),
            test_to: to_email.to_string(),
        }
    }

    pub fn send(&self, to: &str, from: &str, message: String) {
        let email = Message::builder()
            .from(from.parse().unwrap())
            .to(to.parse().unwrap())
            .subject("Test email")
            .header(ContentType::TEXT_PLAIN)
            .body(message)
            .unwrap();

        let response = &self.mailer.send(&email).expect("Email sent successfully");
    }
}
