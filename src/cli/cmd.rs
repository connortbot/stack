use super::args::{
    CheckoutArgs,
    RemoveArgs,
    PushArgs,
    Commands,
};
use crate::error::StackError;
use crate::store::fs::{init, FsStore};
use std::path::Path;
use crate::output::{
    error,
    success,
};

pub struct StackManager {
    store: FsStore,
}

impl StackManager {
    pub fn new(store: FsStore) -> Result<Self, StackError> {
        Ok(Self { store })
    }

    pub fn checkout(&self, args: CheckoutArgs) -> Result<(), StackError> {
        if args.create { 
            self.store.create_stack(&args.name).map_err(|e| {
                error(&e);
                e
            })?;
            success(&format!("Created stack {}", args.name));
        }
        self.store.set_current_stack(&args.name).map_err(|e| {
            error(&e);
            e
        })?;
        success(&format!("Checked out stack {}", args.name));
        Ok(())
    }

    pub fn remove(&self, args: RemoveArgs) -> Result<(), StackError> {
        self.store.remove_stack(&args.name).map_err(|e| {
            error(&e);
            e
        })?;
        self.store.clear_current_stack().map_err(|e| {
            error(&e);
            e
        })?;
        success(&format!("Removed stack {}", args.name));
        Ok(())
    }

    pub fn push(&self, args: PushArgs) -> Result<(), StackError> {
        let current_stack = self.store.push_to_stack(&args.branch).map_err(|e| {
            error(&e);
            e
        })?;
        success(&format!("Pushed branch {} to stack {}", args.branch, current_stack));
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
            Commands::Remove(args) => {
                manager.remove(args)
            }
            Commands::Push(args) => {
                manager.push(args)
            }
        }
    }
}