use crate::todo::prompts;
use crate::todo::todo;
use crate::todo::utils;
use colorize::*;
use eyre::{eyre, Result};
use requestty;

pub fn start() -> Result<()> {
    utils::init();
    println!("{}", "Simple ToDo".cyan());
    loop {
        let cmd = requestty::prompt_one(prompts::main_menu())?;

        match cmd.as_expand_item().unwrap().key {
            'l' => todo::list(),
            'a' => handle_add()?,
            'd' => handle_done()?,
            'r' => handle_remove()?,
            'x' => std::process::exit(0),
            _ => unreachable!(),
        }
    }
}

fn handle_add() -> Result<()> {
    if let Some(title) = requestty::prompt_one(prompts::title())?.as_string() {
        todo::add(title.to_string());
    } else {
        return Err(eyre!("Title is required"));
    }

    Ok(())
}

fn handle_done() -> Result<()> {
    if utils::get_todo_count().unwrap() == 0 {
        println!("{}", "No todos".red());
        return Ok(());
    }

    if let Some(id) = requestty::prompt_one(prompts::id())?.as_string() {
        todo::done(id.to_string());
    } else {
        return Err(eyre!("A valid id is required"));
    }

    Ok(())
}

fn handle_remove() -> Result<()> {
    if utils::get_todo_count().unwrap() == 0 {
        println!("{}", "No todos".red());
        return Ok(());
    }

    if let Some(id) = requestty::prompt_one(prompts::id())?.as_string() {
        todo::remove(id.to_string());
    } else {
        return Err(eyre!("A valid id is required"));
    }
    Ok(())
}
