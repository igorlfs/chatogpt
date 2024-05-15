use rusqlite::{Connection, Result};
use std::error::Error;

pub mod functions;
pub mod model;

pub fn connect() -> Result<Connection, Box<dyn Error>> {
    let connection = Connection::open_in_memory()?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS Chat (
            ChatId integer PRIMARY KEY,
            ChatTitle text,
            CreatedAt text,
            UpdatedAt text
            )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS Message (
            MessageId integer PRIMARY KEY,
            MessageContent text,
            ChatId integer REFERENCES Chats,
            CreatedAt text,
            )",
        [],
    )?;

    Ok(connection)
}
