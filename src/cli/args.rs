use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(name = "stack", about = "PR stack manager for git", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub struct InitArgs {}

#[derive(Parser, Debug)]
pub struct CheckoutArgs {
    pub name: String,
    #[arg(short, long)]
    pub create: bool,
}

#[derive(Parser, Debug)]
pub struct DeleteArgs {
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct PushArgs {
    pub branch: String,
}

#[derive(Parser, Debug)]
pub struct PopArgs {}

#[derive(Parser, Debug)]
pub struct ShiftArgs {}

#[derive(Parser, Debug)]
pub struct ListArgs {}

#[derive(Parser, Debug)]
pub struct StatusArgs {}

#[derive(Parser, Debug)]
pub struct RebaseArgs {
    #[arg(short, long)]
    pub from: Option<usize>,

    #[arg(short, long)]
    pub to: Option<usize>,

    #[arg(long, help = "Rebase the bottom of the stack onto main branch")]
    pub onto_main: bool,
}

#[derive(Parser, Debug)]
pub struct InsertArgs {
    pub branch: String,

    #[arg(short, long)]
    pub index: usize,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    pub index: usize,
}

#[derive(Parser, Debug)]
pub struct ConfigArgs {
    pub setting: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(alias = "i")]
    Init(InitArgs),
    
    #[command(alias = "co")]
    Checkout(CheckoutArgs),

    #[command(alias = "del")]
    Delete(DeleteArgs),

    #[command()]
    Push(PushArgs),

    #[command()]
    Pop(PopArgs),

    #[command()]
    Shift(ShiftArgs),

    #[command()]
    List(ListArgs),

    #[command()]
    Status(StatusArgs),

    #[command()]
    Rebase(RebaseArgs),

    #[command()]
    Insert(InsertArgs),

    #[command()]
    Remove(RemoveArgs),

    #[command()]
    Config(ConfigArgs),
}