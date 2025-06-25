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
    println!("{}", format!("[CONFIRM] {} (y/n/c)", msg).yellow().bold());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let (accept, continue_op) = match input.trim().to_lowercase().as_str() {
        "y" | "yes" => (true, true),
        "n" | "no" => (false, false),
        "c" | "continue" => (false, true),
        _ => (false, false),
    };
    
    Ok((accept, continue_op))
}

pub fn question_bool(msg: &str, default: bool) -> Result<bool, StackError> {
    println!("{}", format!("[?] {} (y/n)", msg).blue().bold());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let response = input.trim().to_lowercase();
    match response.as_str() {
        "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),
        _ => Ok(default),
    }
}

pub fn question_string(msg: &str, default: &str) -> Result<String, StackError> {
    println!("{}", format!("[?] {}", msg).blue().bold());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.is_empty() {
        return Ok(default.to_string());
    } 
    Ok(input.trim().to_string())
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