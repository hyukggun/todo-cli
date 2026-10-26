mod todo;
mod location;
use location::Location;
use todo::Todo;
use std::io::{self, BufRead};

use rusqlite::{Connection, Result};

fn main() {
    
    let conn = Connection::open("todos.db").unwrap();


    println!("Connected to database");
    
    // let path = "schema/todos.sql";
    
    // let stmt = std::fs::read_to_string(path).unwrap_or_else(|err| {
    //     eprintln!("Failed to read schema file: {err:?}");
    //     std::process::exit(1);
    // });
    
    // conn.execute_batch(&stmt).unwrap_or_else(|err| {
    //     eprintln!("Failed to execute schema file: {err:?}");
    //     std::process::exit(1);
    // });

    
    let todo = Todo::new("Test Todo".to_string(), "This is a test 123444".to_string(), Some(Location::new(40.7128, -74.0060)));
    match Todo::insert(&conn, &todo) {
        Ok(_) => println!("Todo inserted successfully"),
        Err(err) => eprintln!("Failed to insert todo: {err:?}"),
    }
    
    match conn.prepare("SELECT * FROM todos").unwrap().query_map((), |row| {
        Todo::from_row(row)
    }) {
        Ok(todos) => todos.for_each(|todo| {
            println!("{}", todo.unwrap());
        }),
        Err(err) => eprintln!("Failed to select todos: {err:?}"),
    }
}
