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
    // pub database_url:
}

impl EnvConfig {
    pub(crate) fn new() -> Self {
        EnvConfig {
            stage: Stage::from(
                dotenvy::var("STAGE").expect("Stage environment variable should be set."),
            ),
        }
    }

    pub(crate) fn is_dev(&self) -> bool {
        self.stage == Stage::Development
    }
    pub(crate) fn is_prod(&self) -> bool {
        self.stage == Stage::Production
    }
}
