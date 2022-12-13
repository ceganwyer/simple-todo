use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub created_at: String,
    pub title: String,
    pub done: bool,
    pub id: Uuid,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub data: Vec<Todo>,
}
