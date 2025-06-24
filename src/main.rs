use clap::Parser;
use cli::args::Cli;

mod cli;
mod store;
mod error;
mod output;
mod git;

fn main() {
    let cli = Cli::parse();
    
    if let Err(_) = cli::cmd::execute(cli.command) {
        std::process::exit(1);
    }
}
