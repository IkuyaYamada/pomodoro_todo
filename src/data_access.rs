use crate::models::TodoItem;
use std::fs::File;
use std::io::{self, Read, Write};

pub fn save(todos: &Vec<TodoItem>, filename: &str) -> io::Result<()> {
    let json = serde_json::to_string(&todos)?;
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load(filename: &str) -> io::Result<Vec<TodoItem>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let todos = serde_json::from_str(&contents)?;
    Ok(todos)
}
