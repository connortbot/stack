
use crate::error::StackError;

#[allow(non_snake_case)]
pub struct Config {
    pub MAIN_BRANCH_NAME: String,
    pub CONFIRMATION_ON_GIT_PUSH: bool,
    pub CONFIRMATION_ON_GIT_REBASE: bool,
}

impl Config {
    pub fn new() -> Self {
        // DEFAULTS
        Self {
            MAIN_BRANCH_NAME: "main".to_string(),
            CONFIRMATION_ON_GIT_PUSH: true,
            CONFIRMATION_ON_GIT_REBASE: true,
        }
    }

    pub fn to_string(&self) -> String {
        format!("MAIN_BRANCH_NAME={}\nCONFIRMATION_ON_GIT_PUSH={}\nCONFIRMATION_ON_GIT_REBASE={}",
            self.MAIN_BRANCH_NAME,
            self.CONFIRMATION_ON_GIT_PUSH,
            self.CONFIRMATION_ON_GIT_REBASE,
        )
    }

    pub fn set_kv(&mut self, key: &str, value: &str) {
        match key {
            "MAIN_BRANCH_NAME" => {
                self.MAIN_BRANCH_NAME = value.to_string();
            }
            "CONFIRMATION_ON_GIT_PUSH" => {
                self.CONFIRMATION_ON_GIT_PUSH = value == "true" || value == "1";
            }
            "CONFIRMATION_ON_GIT_REBASE" => {
                self.CONFIRMATION_ON_GIT_REBASE = value == "true" || value == "1";
            }
            _ => {}
        }
    }

    pub fn from_string(contents: String) -> Result<Self, StackError> {
        let lines = contents.lines();
        let mut config = Config::new();

        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            
            let parts = line.splitn(2, '=').collect::<Vec<&str>>();
            if parts.len() != 2 {
                continue;
            }
            
            let key = parts[0].trim();
            let value = parts[1].trim();
            
            config.set_kv(key, value);
        }
        
        Ok(config)
    }
}