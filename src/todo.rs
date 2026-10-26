use crate::location::Location;
use std::io::{stdin, stdout, Write};
use uuid::Uuid;

use rusqlite::{Connection, Row, params};

pub struct Todo {
    id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub location: Option<Location>,
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {}, Title: {}, Description: {}, Completed: {}, Location: {:?}",
            self.id, self.title, self.description, self.completed, self.location
        )
    }
}

impl Todo {
    pub fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        let temp_id = row.get::<_, String>("id")?;
        let _id = Uuid::parse_str(&temp_id)
        .map_err(|err| rusqlite::Error::FromSqlConversionFailure(
            0,
            rusqlite::types::Type::Text,
            Box::new(err)
            )
        )?;
        let description = row.get::<_, String>("description")?;
        let title = row.get::<_, String>("title")?;
        let latitude = row.get::<_, f64>("latitude")?;
        let longitude = row.get::<_, f64>("longitude")?;
        let location = Location::new(latitude, longitude);
        let completed = row.get::<_, bool>("completed")?;
        Ok(Todo::new(title, description, Some(location)))
    }
    
    pub fn insert(conn: &Connection, todo: &Todo) -> rusqlite::Result<()> {
        let location = match &todo.location {
            Some(location) => location,
            None => return Err(rusqlite::Error::QueryReturnedNoRows),
        };

        let mut stmt = conn.prepare("INSERT INTO todos (id, title, description, latitude, longitude, completed) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?;
        stmt.execute((
            todo.id.to_string(),
            &todo.title,
            &todo.description,
            location.latitude,
            location.longitude,
            todo.completed,
        ))?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum TodoError {
    IoError(std::io::Error),
    ParseFloatError(std::num::ParseFloatError),
}

pub type TodoResult<T> = Result<T, TodoError>;

impl From<std::io::Error> for TodoError {
    fn from(error: std::io::Error) -> Self {
        TodoError::IoError(error)
    }
}

impl From<std::num::ParseFloatError> for TodoError {
    fn from(error: std::num::ParseFloatError) -> Self {
        TodoError::ParseFloatError(error)
    }
}

impl Todo {
    pub fn new(title: String, description: String, location: Option<Location>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            completed: false,
            location,
        }
    }

    fn prompt_line(label: &str) -> TodoResult<String> {
        print!("{label}");
        stdout().flush()?;
        let mut line = String::new();
        if stdin().read_line(&mut line)? == 0 {
            return Err(TodoError::IoError(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                format!("Failed to read {label}"),
            )));
        }
        Ok(line.trim_end().to_string())
    }

    pub fn new_from_input() -> TodoResult<Self> {
        let title = Self::prompt_line("Enter title: ")?;
        let description = Self::prompt_line("Enter description: ")?;
        let latitude = Self::prompt_line("Enter location latitude: ")?;
        let longitude = Self::prompt_line("Enter location longitude: ")?;
        let location = Location::from_strings(&latitude, &longitude)?;
        Ok(Self::new(title, description, Some(location)))
    }
}
