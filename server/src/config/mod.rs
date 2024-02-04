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
    // pub database_url:
}

impl EnvConfig {
    pub(crate) fn new() -> Self {
        let stage =
            Stage::from(dotenvy::var("STAGE").expect("Stage environment variable should be set."));
        let frontend_url = Self::get_frontend_url(&stage);

        return EnvConfig {
            stage,
            frontend_url,
        };
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
