use crate::todo::structs::Todo;
use crate::todo::utils;
use chrono::prelude::*;
use colorize::*;

pub fn add(title: String) {
    if title.is_empty() {
        println!("{}", "No title provided".red());
        return;
    }

    let mut todos = utils::get_todos().unwrap();

    let todo = Todo {
        created_at: utils::get_timestamp(),
        title,
        done: false,
        id: utils::get_id(),
        updated_at: utils::get_timestamp(),
    };

    todos.push(todo);

    utils::save_todos(todos);

    println!("{}", "Added todo".green());
}

pub fn list() {
    let todos = utils::get_todos().unwrap();

    if todos.is_empty() {
        println!("{}", "No todos".red());
        return;
    }

    println!(
        "{0: <10} | {1: <20} | {2: <20} | {3: <20} | {4: <20}",
        "ID", "Title", "Created", "Updated", "Done"
    );

    println!();

    for todo in todos {
        let id = todo.id.simple().to_string();
        let id = id.get(27..).unwrap();
        let created = todo.created_at.parse::<DateTime<Local>>().unwrap();
        let updated = todo.updated_at.parse::<DateTime<Local>>().unwrap();

        println!(
            "{0: <10} | {1: <20} | {2: <20} | {3: <20} | {4: <20}",
            id,
            todo.title,
            utils::get_simple_timestamp(created),
            utils::get_simple_timestamp(updated),
            if todo.done { "✔️" } else { "❌" }
        );
    }
}

pub fn done(id: String) {
    let mut todos = utils::get_todos().unwrap();

    if !exists(id.as_str()) || id.is_empty() {
        println!("{}", "Todo not found".red());
        return;
    }

    for todo in &mut todos {
        if todo.id.simple().to_string().contains(id.as_str()) {
            todo.done = true;
            todo.updated_at = utils::get_timestamp();
        }
    }

    utils::save_todos(todos);

    println!("{}", "Marked todo as done".green())
}

pub fn remove(id: String) {
    let mut todos = utils::get_todos().unwrap();

    if !exists(id.as_str()) || id.is_empty() {
        println!("{}", "Todo not found".red());
        return;
    }

    todos.retain(|todo| !todo.id.simple().to_string().contains(id.as_str()));

    utils::save_todos(todos);

    println!("{}", "Removed todo".green())
}

pub fn exists(id: &str) -> bool {
    let todos = utils::get_todos().unwrap().clone();

    todos
        .iter()
        .any(|todo| todo.id.simple().to_string().contains(id))
}
