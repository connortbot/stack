
use std::path::{Path, PathBuf};
use crate::error::StackError;
use crate::output::{error, success, info};
use std::fs;

const STACK_DIR: &str = ".stack";
const CURRENT_STACK_PATH: &str = "current";
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
    root_dir: PathBuf,
    stacks_dir: PathBuf,
    current_stack: PathBuf,
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
        let current_stack = stack_dir.join(CURRENT_STACK_PATH);

        fs::create_dir_all(&stacks_dir)?;
        Ok(Self { root_dir, stacks_dir, current_stack })
    }

    fn find_repository_root(start_dir: &Path) -> Result<PathBuf, StackError> {
        let mut current = start_dir.to_path_buf();
        loop {
            let data_dir = current.join(STACK_DIR);
            if data_dir.exists() {
                return Ok(current);
            }
            
            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
            } else {
                return Err(StackError::NotFound(
                    "No .stack directory found. Run `stack init` to create one.".to_string()
                ));
            }
        }
    }

    fn get_stack_path(&self, stack_name: &str) -> PathBuf {
        self.stacks_dir.join(stack_name)
    }

    pub fn create_stack(&self, stack_name: &str) -> Result<(), StackError> {
        let stack_dir = self.get_stack_path(stack_name);
        if stack_dir.exists() {
            return Err(StackError::Invalid(format!("Stack {} already exists.", stack_name)));
        }
        fs::write(stack_dir, "")?;
        Ok(())
    }

    pub fn set_current_stack(&self, stack_name: &str) -> Result<(), StackError> {
        let stack_dir = self.get_stack_path(stack_name);
        if !stack_dir.exists() {
            return Err(StackError::Invalid(format!("Stack {} does not exist.", stack_name)));
        }
        fs::write(&self.current_stack, stack_name)?;
        Ok(())
    }
}
