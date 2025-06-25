use super::args::{
    CheckoutArgs,
    DeleteArgs,
    PushArgs,
    PopArgs,
    ShiftArgs,
    ListArgs,
    StatusArgs,
    RebaseArgs,
    InsertArgs,
    RemoveArgs,
    ConfigArgs,
    Commands,
};
use crate::error::StackError;
use crate::store::fs::{init, FsStore};
use crate::git::git::Git;
use crate::output::{
    error,
    success,
    info,
    confirm,
    show_stacks,
    show_stack,
};
use crate::config::config::Config;

pub struct StackManager {
    store: FsStore,
    git: Git,
    config: Config,
}

impl StackManager {
    pub fn new(store: FsStore, git: Git, config: Config) -> Result<Self, StackError> {
        Ok(Self { store, git, config })
    }

    fn configured_confirmation(&self, msg: &str, config_condition: bool, skip_confirmation: bool) -> Result<(bool, bool), StackError> {
        if config_condition && !skip_confirmation {
            let confirmation = confirm(&format!("{}", msg))?;
            Ok(confirmation)
        } else {
            Ok((true, true))
        }
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

    pub fn shift(&self, _args: ShiftArgs) -> Result<(), StackError> {
        let first_branch = self.store.shift_from_stack().map_err(|e| {
            error(&e);
            e
        })?;
        success(&format!("Shifted branch {} from stack", first_branch));
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

    pub fn rebase(&self, args: RebaseArgs) -> Result<(), StackError> {
        let current_stack = self.store.get_current_stack_path().map_err(|e| {
            error(&e);
            e
        })?;

        let stack_contents = self.store.get_stack_contents(&current_stack).map_err(|e| {
            error(&e);
            e
        })?;

        if stack_contents.len() == 0 {
            success("No branches in stack");
            return Ok(());
        }

        let last_index = stack_contents.len() - 1;
        let from = args.from.unwrap_or(0).min(last_index);
        let to = args.to.unwrap_or(last_index).min(last_index);

        if args.onto_main && from == 0 {
            let (accept, continue_op) = self.configured_confirmation(
                &format!("Rebase on {} from {}?", self.config.MAIN_BRANCH_NAME, stack_contents[0]),
                self.config.CONFIRMATION_ON_GIT_REBASE,
                args.yes
            )?;

            if !continue_op { return Ok(()); }
            if accept {
                info(&format!("Pulling {}...", self.config.MAIN_BRANCH_NAME));
                self.git.checkout(&self.config.MAIN_BRANCH_NAME).map_err(|e| {
                    error(&e);
                    e
                })?;
                self.git.pull().map_err(|e| {
                    error(&e);
                    e
                })?;
                info("Rebasing...");
                self.git.rebase_onto(&stack_contents[0], &self.config.MAIN_BRANCH_NAME).map_err(|e| {
                    error(&e);
                    e
                })?;

                let (accept, continue_op) = self.configured_confirmation(
                    &format!("Push changes to {}?", stack_contents[0]),
                    self.config.CONFIRMATION_ON_GIT_PUSH,
                    args.yes
                )?;

                if !continue_op { return Ok(()); }
                if accept {
                    info(&format!("Pushing changes to {}", stack_contents[0]));
                    self.git.push(true).map_err(|e| {
                        error(&e);
                        e
                    })?;
                }
            }
        }

        for window in stack_contents[from..=to].windows(2) {
            let base_branch = &window[0];
            let target_branch = &window[1];
            
            let (accept, continue_op) = self.configured_confirmation(
                &format!("Rebase {} onto {}?", target_branch, base_branch),
                self.config.CONFIRMATION_ON_GIT_REBASE,
                args.yes
            )?;

            if !continue_op { return Ok(()); }
            if accept {
                info(&format!("Rebasing {} onto {}", target_branch, base_branch));
                self.git.rebase_onto(target_branch, base_branch).map_err(|e| {
                    error(&e);
                    e
                })?;

                let (accept, continue_op) = self.configured_confirmation(
                    &format!("Push changes to {}?", target_branch),
                    self.config.CONFIRMATION_ON_GIT_PUSH,
                    args.yes
                )?;

                if !continue_op { return Ok(()); }
                if accept {
                    info(&format!("Pushing changes to {}", target_branch));
                    self.git.push(true).map_err(|e| {
                        error(&e);
                        e
                    })?;
                }
            }
        }

        success("Stack rebased successfully");
        Ok(())
    }

    pub fn insert(&self, args: InsertArgs) -> Result<(), StackError> {
        if !self.git.check_branch_exists(&args.branch).map_err(|e| {
            error(&e);
            e
        })? {
            let err = StackError::Invalid(format!("Branch {} does not exist.", args.branch));
            error(&err);
            return Err(err);
        }

        self.store.insert_into_stack(&args.branch, args.index).map_err(|e| {
            error(&e);
            e
        })?;
        success(&format!("Inserted branch {} at index {}", args.branch, args.index));
        Ok(())
    }

    pub fn remove(&self, args: RemoveArgs) -> Result<(), StackError> {
        self.store.remove_from_stack(args.index).map_err(|e| {
            error(&e);
            e
        })?;
        success(&format!("Removed branch at index {}", args.index));
        Ok(())
    }

    pub fn config(&self, args: ConfigArgs) -> Result<(), StackError> {
        let parts: Vec<&str> = args.setting.splitn(2, '=').collect();
        if parts.len() != 2 {
            let err = StackError::Invalid("Config setting must be in KEY=VALUE format".to_string());
            error(&err);
            return Err(err);
        }

        let key = parts[0].trim();
        let value = parts[1].trim();

        self.store.update_config(key, value).map_err(|e| {
            error(&e);
            e
        })?;
        
        success(&format!("Updated config with {} = {}", key, value));
        Ok(())
    }
}

pub fn execute(cmd: Commands) -> Result<(), StackError> {
    let current_dir = std::env::current_dir()?;
    
    if let Commands::Init(_) = cmd {
        init(&current_dir);
        Ok(())
    } else {
        let store = FsStore::new(&current_dir)?;
        let git = Git::new();
        let config = store.read_config_file()?;
        let manager = StackManager::new(store, git, config)?;
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
            Commands::Shift(args) => {
                manager.shift(args)
            }
            Commands::List(args) => {
                manager.list(args)
            }
            Commands::Status(args) => {
                manager.status(args)
            }
            Commands::Rebase(args) => {
                manager.rebase(args)
            }
            Commands::Insert(args) => {
                manager.insert(args)
            }
            Commands::Remove(args) => {
                manager.remove(args)
            }
            Commands::Config(args) => {
                manager.config(args)
            }
        }
    }
}