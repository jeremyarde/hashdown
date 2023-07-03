#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Stage {
    DEV,
    PROD,
}

impl Stage {
    pub(crate) fn from(var: String) -> Stage {
        match var.to_lowercase().as_str() {
            "dev" => Stage::DEV,
            "prod" => Stage::PROD,
            _ => panic!("STAGE environment variable not available"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnvConfig {
    pub(crate) stage: Stage,
}

impl EnvConfig {
    pub(crate) fn new() -> Self {
        return EnvConfig {
            stage: Stage::from(
                dotenvy::var("STAGE").expect("Stage environment variable should be set."),
            ),
        };
    }

    pub(crate) fn is_dev(&self) -> bool {
        if self.stage == Stage::DEV {
            true
        } else {
            false
        }
    }
}
