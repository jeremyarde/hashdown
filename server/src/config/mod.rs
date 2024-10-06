#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Stage {
    Development,
    Production,
}

impl Stage {
    pub(crate) fn from(var: String) -> Stage {
        match var.to_lowercase().as_str() {
            "development" => Stage::Development,
            "production" => Stage::Production,
            _ => panic!("STAGE environment variable not available"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnvConfig {
    pub(crate) stage: Stage,
    pub frontend_url: String,
    // pub stripe_success_url: String,
    // pub stripe_cancel_url: String,
    pub stripe_secret_key: String,
    pub database_url: String,
    pub smtp_username: String,
    pub smtp_password: String,
    // pub database_url:
}

impl EnvConfig {
    pub(crate) fn new() -> Self {
        if !dotenvy::dotenv().is_ok() {
            dotenvy::from_filename("./server/.env").expect("Could not load .env file");
        }

        let stage =
            Stage::from(dotenvy::var("STAGE").expect("Stage environment variable should be set."));
        let frontend_url = Self::get_frontend_url(&stage);
        let stripe_secret_key =
            dotenvy::var("STRIPE_SECRETKEY").expect("Stripe secret key not set");
        let database_url = dotenvy::var("DATABASE_URL").expect("Database url not set");
        let smtp_username = dotenvy::var("SMTP_USERNAME").expect("SMTP username not set");
        let smtp_password = dotenvy::var("SMTP_PASSWORD").expect("SMTP password not set");

        EnvConfig {
            stage,
            frontend_url,
            stripe_secret_key,
            database_url,
            smtp_username,
            smtp_password,
        }
    }

    pub fn get_frontend_url(stage: &Stage) -> String {
        match stage {
            Stage::Development => "localhost:5173".to_string(),
            Stage::Production => "https://gethashdown.com".to_string(),
        }
    }

    pub(crate) fn is_dev(&self) -> bool {
        self.stage == Stage::Development
    }
    pub(crate) fn is_prod(&self) -> bool {
        self.stage == Stage::Production
    }
}
