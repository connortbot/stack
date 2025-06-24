use colored::*;
use crate::error::StackError;

pub fn error(err: &StackError) {
    for line in err.to_string().lines() {
        eprintln!("{} {}", "[ERROR]".red().bold(), line);
    }
}

pub fn success(msg: &str) {
    println!("{} {}", "[SUCCESS]".green().bold(), msg);
}

pub fn info(msg: &str) {
    println!("{} {}", "[INFO]".blue().bold(), msg);
}

pub fn show_stacks(current_stack: &str, stacks: &Vec<String>) {
    if stacks.is_empty() {
        info("No stacks found");
        return;
    }
    for (index, stack) in stacks.iter().enumerate() {
        if stack == current_stack {
            println!("{}", format!("* {}: {}", index, stack).green().bold());
        } else {
            println!("{}: {}", format!("  {}", index).blue().bold(), stack);
        }
    }
}

pub fn show_stack(list: &Vec<String>) {
    if list.is_empty() {
        info("Stack is empty");
        return;
    }
    for (index, branch) in list.iter().enumerate() {
        println!("{}: {}", format!("[{}]", index).blue().bold(), branch);
    }
}

pub fn warning(msg: &str) {
    println!("{} {}", "[WARNING]".yellow().bold(), msg);
}