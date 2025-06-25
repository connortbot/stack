use colored::*;
use crate::error::StackError;
use std::io::{self, Write};

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

pub fn confirm(msg: &str) -> Result<(bool, bool), StackError> {
    println!("{} {}", "[CONFIRM] (y/n/c)".yellow().bold(), msg);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let (accept, continue_op) = match input.trim().to_lowercase().as_str() {
        "y" | "yes" => (true, true),   // accept and continue
        "n" | "no" => (false, false),  // don't accept and don't continue
        "c" | "continue" => (false, true), // don't accept but continue
        _ => (false, false),
    };
    
    Ok((accept, continue_op))
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