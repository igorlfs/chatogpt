use rusqlite::{Connection, Result};
use std::error::Error;

pub mod functions;
pub mod model;

pub fn connect() -> Result<Connection, Box<dyn Error>> {
    let connection =
        Connection::open_in_memory().expect("Falha ao abrir banco de dados na memória!");

    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS Chat (
            ChatId integer PRIMARY KEY AUTOINCREMENT,
            ChatTitle text,
            CreatedAt text,
            UpdatedAt text
            )",
            [],
        )
        .expect("Falha ao criar tabela de chats!");

    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS Message (
            MessageId integer PRIMARY KEY AUTOINCREMENT,
            MessageContent text,
            Role text,
            ChatId integer REFERENCES Chat,
            CreatedAt text
            )",
            [],
        )
        .expect("Falha ao criar tabela de mensagens!");

    Ok(connection)
}
