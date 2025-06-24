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
pub struct ListArgs {}

#[derive(Parser, Debug)]
pub struct StatusArgs {}

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
    List(ListArgs),

    #[command()]
    Status(StatusArgs),
}