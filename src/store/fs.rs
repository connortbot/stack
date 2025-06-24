
use std::path::{Path, PathBuf};
use crate::error::StackError;
use crate::output::{error, success, info};
use std::fs;

const STACK_DIR: &str = ".stack";
const STACKS_DIR: &str = "stacks";

pub fn init(path_dir: &Path) {
    let stack_dir = path_dir.join(STACK_DIR);
    if !stack_dir.exists() {
        let stacks_dir = stack_dir.join(STACKS_DIR);
        fs::create_dir_all(&stacks_dir).unwrap();
        success("Stack directory created successfully!");
    } else {
        error(&StackError::Invalid(format!("Stack directory already exists!")));
    }
}

pub struct FsStore {
    root_dir: PathBuf
}

impl FsStore {
    pub fn new(start_dir: &Path) -> Result<Self, StackError> {
        let root_dir = Self::find_repository_root(start_dir)
        .map_err(|e| {
            error(&e);
            e
        })?;
        info(&format!("Stack directory found at: {:?}", root_dir));
        let stack_dir = root_dir.join(STACK_DIR);
        let stacks_dir = stack_dir.join(STACKS_DIR);

        fs::create_dir_all(&stacks_dir)?;
        Ok(Self { root_dir: start_dir.to_path_buf() })
    }

    fn find_repository_root(start_dir: &Path) -> Result<PathBuf, StackError> {
        info(&format!("Finding repository root in: {:?}", start_dir));
        let mut current = start_dir.to_path_buf();
        loop {
            let data_dir = current.join(STACK_DIR);
            if data_dir.exists() {
                return Ok(current);
            }
            
            if let Some(parent) = current.parent() {
                info(&format!("Checking parent directory: {:?}", parent));
                current = parent.to_path_buf();
            } else {
                return Err(StackError::NotFound(
                    "No .stack directory found. Run `stack init` to create one.".to_string()
                ));
            }
        }
    }
}
