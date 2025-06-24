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
pub struct RemoveArgs {
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct PushArgs {
    pub branch: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(alias = "i")]
    Init(InitArgs),
    
    #[command(alias = "co")]
    Checkout(CheckoutArgs),

    #[command(alias = "rm")]
    Remove(RemoveArgs),

    #[command()]
    Push(PushArgs),
}