use functions;

#[cgf(tests)]
mod tests {
    use rusqlite::{params, Connection, Result};
    fn setup_db_in_memory() -> Result<Connection> {
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

        Ok(connection)
    }

    #[test]
    fn test_create_message() -> Result<()>{
        let connection = setup_in_memory_db()?;
        
        let chat = Chat::new();

        let message = 

        functions::create_message(connection, chat_id, message);

        Ok(())
    }
    }
}
