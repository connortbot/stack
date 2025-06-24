use colored::*;
use crate::error::StackError;

pub fn error(err: &StackError) {
    eprintln!("{} {}", "[ERROR]".red().bold(), err);
}

pub fn success(msg: &str) {
    println!("{} {}", "[SUCCESS]".green().bold(), msg);
}

pub fn info(msg: &str) {
    println!("{} {}", "[INFO]".blue().bold(), msg);
}

pub fn warning(msg: &str) {
    println!("{} {}", "[WARNING]".yellow().bold(), msg);
}