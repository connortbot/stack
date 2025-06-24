use super::args::{
    CheckoutArgs,
    DeleteArgs,
    PushArgs,
    PopArgs,
    ListArgs,
    StatusArgs,
    Commands,
};
use crate::error::StackError;
use crate::store::fs::{init, FsStore};
use crate::git::git::Git;
use std::path::Path;
use crate::output::{
    error,
    success,
    show_stacks,
    show_stack,
};

pub struct StackManager {
    store: FsStore,
    git: Git,
}

impl StackManager {
    pub fn new(store: FsStore, git: Git) -> Result<Self, StackError> {
        Ok(Self { store, git })
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

    pub fn delete(&self, args: DeleteArgs) -> Result<(), StackError> {
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
        if !self.git.check_branch_exists(&args.branch).map_err(|e| {
            error(&e);
            e
        })? {
            let err = StackError::Invalid(format!("Branch {} does not exist.", args.branch));
            error(&err);
            return Err(err);
        }

        let current_stack = self.store.get_current_stack_path().map_err(|e| {
            error(&e);
            e
        })?;

        let stack_contents = self.store.get_stack_contents(&current_stack).map_err(|e| {
            error(&e);
            e
        })?;
        if stack_contents.contains(&args.branch) {
            let err = StackError::Invalid(format!("Branch {} already in stack.", args.branch));
            error(&err);
            return Err(err);
        }

        self.store.push_to_stack(&args.branch).map_err(|e| {
            error(&e);
            e
        })?;
        success(&format!("Pushed branch {} to stack {}", args.branch, current_stack));
        Ok(())
    }

    pub fn pop(&self, _args: PopArgs) -> Result<(), StackError> {
        let last_branch = self.store.pop_from_stack().map_err(|e| {
            error(&e);
            e
        })?;
        success(&format!("Popped branch {} from stack", last_branch));
        Ok(())
    }

    pub fn list(&self, _args: ListArgs) -> Result<(), StackError> {
        let current_stack = self.store.get_current_stack_path().unwrap_or_default();
        let stacks = self.store.get_stacks().map_err(|e| {
            error(&e);
            e
        })?;
        show_stacks(&current_stack, &stacks);
        Ok(())
    }

    pub fn status(&self, _args: StatusArgs) -> Result<(), StackError> {
        let current_stack = self.store.get_current_stack_path().map_err(|e| {
            error(&e);
            e
        })?;
        
        let stack_contents = self.store.get_stack_contents(&current_stack).map_err(|e| {
            error(&e);
            e
        })?;
        show_stack(&stack_contents);
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
        let git = Git::new();
        let manager = StackManager::new(store, git)?;
        match cmd {
            Commands::Init(_) => unreachable!(),
            Commands::Checkout(args) => {
                manager.checkout(args)
            }
            Commands::Delete(args) => {
                manager.delete(args)
            }
            Commands::Push(args) => {
                manager.push(args)
            }
            Commands::Pop(args) => {
                manager.pop(args)
            }
            Commands::List(args) => {
                manager.list(args)
            }
            Commands::Status(args) => {
                manager.status(args)
            }
        }
    }
}