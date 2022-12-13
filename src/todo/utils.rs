use crate::todo::structs;
use chrono::prelude::*;
use colorize::*;
use home;
use serde_json::{from_str, Result};
use std::{fs, io::Write, path::PathBuf};
use uuid::Uuid;

const DATA_DIR: &str = ".todo";
const DATA_FILE: &str = "data";

pub fn init() {
    if let Some(mut path) = home::home_dir() {
        path.push(DATA_DIR);
        if !fs::metadata(&path).is_ok() {
            // Create directory.
            fs::create_dir(&path).unwrap();
            println!("{} {:?}", "Created directory:".green(), path.as_os_str());
        }
        path.push(DATA_FILE);
        if !fs::metadata(&path).is_ok() {
            // Create file
            let mut file = fs::File::create(&path).unwrap();
            // Write to file
            file.write_all(b"{\"data\": []}").unwrap();

            println!("{} {:?}", "Created file:".green(), path.as_os_str());
        }
    } else {
        panic!("Couldn't find home directory!");
    }
}

pub fn get_timestamp() -> String {
    let now = chrono::Local::now();
    let timestamp = now.format("%+").to_string();

    timestamp
}

pub fn get_simple_timestamp(ts: DateTime<Local>) -> String {
    let timestamp = ts.format("%m-%d %H:%M").to_string();

    timestamp
}

pub fn get_id() -> Uuid {
    Uuid::new_v4()
}

pub fn get_todos() -> Result<Vec<structs::Todo>> {
    let data = fs::read_to_string(get_data_file_path()).unwrap();
    let todos: structs::Config = from_str(&data)?;

    Ok(todos.data)
}

pub fn get_todo_ids() -> Result<Vec<Uuid>> {
    let data = fs::read_to_string(get_data_file_path()).unwrap();
    let todos: structs::Config = from_str(&data)?;

    Ok(todos.data.iter().map(|todo| todo.id).collect())
}

pub fn save_todos(todos: Vec<structs::Todo>) {
    let config_file = structs::Config { data: todos };
    let json = serde_json::to_string(&config_file).unwrap();

    let mut file = fs::File::create(get_data_file_path()).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

fn get_data_file_path() -> PathBuf {
    let mut path = home::home_dir().unwrap();
    path.push(DATA_DIR);
    path.push(DATA_FILE);
    path
}
