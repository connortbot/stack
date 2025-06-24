use crate::error::StackError;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use crate::output::{error, warning};

fn run_command(cmd: &str, args: &[&str]) -> Result<(), StackError> {
    let mut child = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| StackError::Git(format!("Failed to execute git command: {}", e)))?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let mut error_message = String::new();

    let stdout_reader = BufReader::new(stdout);
    for line in stdout_reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(e) => return Err(StackError::Git(format!("Failed to read git output: {}", e)))
        }
    }

    let stderr_reader = BufReader::new(stderr);
    for line in stderr_reader.lines() {
        match line {
            Ok(line) => {
                warning(&line);
                error_message.push_str(&line);
                error_message.push('\n');
            }
            Err(e) => return Err(StackError::Git(format!("Failed to read git error output: {}", e)))
        }
    }

    let status = child.wait()
        .map_err(|e| StackError::Git(format!("Failed to wait for git command: {}", e)))?;

    if !status.success() {
        if error_message.is_empty() {
            error_message = "Git command failed".to_string();
        }
        return Err(StackError::Git(error_message.trim().to_string()));
    }

    Ok(())
}

pub struct Git {}

impl Git {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_branch_exists(&self, branch_name: &str) -> Result<bool, StackError> {
        let result = Command::new("git")
            .args(["branch", "--list", branch_name])
            .output()
            .map_err(|e| {
                let err = StackError::Git(format!("Failed to check branch existence: {}", e));
                error(&err);
                err
            })?;

        let output = String::from_utf8_lossy(&result.stdout);
        Ok(!output.trim().is_empty()) // empty output means branch doesn't exist
    }

    pub fn checkout(&self, branch_name: &str) -> Result<(), StackError> {
        if !self.check_branch_exists(branch_name)? {
            let err = StackError::Invalid(format!("Branch {} does not exist", branch_name));
            error(&err);
            return Err(err);
        }

        run_command("git", &["checkout", branch_name])
    }

    pub fn rebase(&self, target_branch: &str) -> Result<(), StackError> {
        if !self.check_branch_exists(target_branch)? {
            let err = StackError::Invalid(format!("Target branch {} does not exist", target_branch));
            error(&err);
            return Err(err);
        }

        run_command("git", &["rebase", "--committer-date-is-author-date", target_branch])
    }

    pub fn rebase_onto(&self, target_branch: &str, base_branch: &str) -> Result<(), StackError> {
        if !self.check_branch_exists(target_branch)? {
            let err = StackError::Invalid(format!("Target branch {} does not exist", target_branch));
            error(&err);
            return Err(err);
        }
        if !self.check_branch_exists(base_branch)? {
            let err = StackError::Invalid(format!("Base branch {} does not exist", base_branch));
            error(&err);
            return Err(err);
        }

        self.checkout(base_branch)?;
        self.rebase(target_branch)
    }
}