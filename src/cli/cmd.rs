use super::args::{
    CheckoutArgs,
    Commands,
};
use crate::error::StackError;
use crate::store::fs::{init, FsStore};
use std::path::Path;

pub struct StackManager {
    store: FsStore,
}

impl StackManager {
    pub fn new(store: FsStore) -> Result<Self, StackError> {
        Ok(Self { store })
    }

    pub fn checkout(&self, args: CheckoutArgs) -> Result<(), StackError> {
        Ok(())
    }
}

pub fn execute(cmd: Commands) -> Result<(), StackError> {
    let current_dir = std::env::current_dir()?;
    
    if let Commands::Init(_) = cmd {
        init(Path::new("."));
        Ok(())
    } else {
        let store = FsStore::new(&current_dir)?;
        let manager = StackManager::new(store)?;
        match cmd {
            Commands::Init(_) => unreachable!(),
            Commands::Checkout(args) => {
                manager.checkout(args)
            }
        }
    }
}