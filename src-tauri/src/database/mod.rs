pub mod functions;
pub mod model;

use rusqlite::Connection;

pub fn connect() -> Connection {
    let connection = Connection::open_in_memory().expect("Erro ao iniciar base de dados");

    connection
        .execute(
            r"CREATE TABLE IF NOT EXISTS Chat (
            ChatId integer PRIMARY KEY AUTOINCREMENT,
            ChatTitle text
            )",
            [],
        )
        .expect("Falha ao criar tabela de chats!");

    connection
        .execute(
            r"CREATE TABLE IF NOT EXISTS Message (
            MessageId integer PRIMARY KEY AUTOINCREMENT,
            MessageContent text,
            Role text,
            ChatId integer,
            CreatedAt text
            )",
            [],
        )
        .expect("Falha ao criar tabela de mensagens!");

    connection
}
