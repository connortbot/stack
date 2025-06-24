use crate::error::StackError;
use std::process::Command;

pub struct Git {}

impl Git {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_branch_exists(&self, branch_name: &str) -> Result<bool, StackError> {
        let result = Command::new("git")
            .args(["branch", "--list", branch_name])
            .output()
            .map_err(|e| StackError::Git(e.to_string()))?;

        let output = String::from_utf8_lossy(&result.stdout);
        Ok(!output.trim().is_empty()) // empty output means branch doesn't exist
    }
}