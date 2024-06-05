use super::model::{Chat, Message};
use rusqlite::Connection;
use std::error::Error;

pub fn get_chat(connection: &Connection, chat_id: u32) -> Result<Chat, Box<dyn Error>> {
    let mut query =
        connection.prepare(format!("SELECT * FROM Chat WHERE ChatId = {chat_id}").as_str())?;

    let mut query_result = query.query_map([], |row| {
        Ok(Chat {
            id: row.get("ChatId")?,
            title: row.get("ChatTitle")?,
            messages: vec![],
        })
    })?;

    let mut chat = query_result.next().unwrap().unwrap();
    chat.messages = get_chat_messages(connection, chat_id, None).unwrap();
    Ok(chat)
}

pub fn create_chat(title: &str, connection: &Connection) -> u32 {
    let mut query = connection
        .prepare(format!("INSERT INTO Chat (ChatTitle) VALUES ('{title}')").as_str())
        .expect("Erro ao criar nova conversa");
    query.insert([]).expect("Erro ao recuperar ID de inserção") as u32
}

pub fn delete_chat(connection: &Connection, chat_id: u32) -> Result<(), Box<dyn Error>> {
    connection
        .prepare(format!("DELETE FROM Message WHERE ChatId = {chat_id}").as_str())?
        .execute([])?;
    connection
        .prepare(format!("DELETE FROM Chat WHERE ChatId = {chat_id}").as_str())?
        .execute([])?;
    Ok(())
}

pub fn get_all_chats(connection: &Connection) -> Result<Vec<Chat>, Box<dyn Error>> {
    let mut query = connection.prepare("SELECT * FROM Chat")?;
    let query_result = query.query_map([], |row| {
        Ok(Chat {
            id: row.get("ChatId")?,
            title: row.get("ChatTitle")?,
            messages: vec![],
        })
    })?;
    Ok(query_result
        .map(|opt| {
            let mut chat = opt.unwrap();
            chat.messages = get_chat_messages(connection, chat.id, None).unwrap();
            chat
        })
        .collect())
}

pub fn get_chat_messages(
    connection: &Connection,
    chat_id: u32,
    limit: Option<u32>,
) -> Result<Vec<Message>, Box<dyn Error>> {
    let mut query = if let Some(limit) = limit {
        connection.prepare(
            format!("SELECT * FROM Message WHERE ChatId = {chat_id} ORDER BY CreatedAt DESC LIMIT {limit}")
                .as_str()
        )?
    } else {
        connection.prepare(
            format!("SELECT * FROM Message WHERE ChatId = {chat_id} ORDER BY CreatedAt DESC")
                .as_str(),
        )?
    };

    let query_result = query.query_map([], |row| {
        Ok(Message {
            id: row.get("MessageId")?,
            content: row.get("MessageContent")?,
            role: row.get("Role")?,
            created_at: row.get("CreatedAt")?,
        })
    })?;

    Ok(query_result.map(|opt| opt.unwrap()).collect())
}

pub fn create_message(
    connection: &Connection,
    chat_id: u32,
    message: &Message,
) -> Result<(), Box<dyn Error>> {
    connection.prepare(
        format!(
            "INSERT INTO Message (MessageContent, Role, ChatId, CreatedAt) VALUES ('{}', '{}', {}, '{}')",
            message.content.replace('\'', ""), // NOTE: not ideal
            message.role,
            chat_id,
            message.created_at.format("%+"),
        )
            .as_str(),
    )?.execute([])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::database::functions::{
        create_chat, create_message, delete_chat, get_all_chats, get_chat, get_chat_messages,
    };
    use crate::database::model::{Chat, Message};
    use rusqlite::{Connection, Result};

    fn setup_db_in_memory() -> Result<Connection> {
        let connection = Connection::open_in_memory()?;
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
    fn test_create_message() -> Result<()> {
        let connection = setup_db_in_memory()?;

        let chat = Chat {
            id: 1,
            title: "chat test".to_string(),
            messages: Vec::new(),
        };

        let message = Message {
            id: 1,
            role: "User".to_string(),
            content: "This is a test message".to_string(),
            created_at: chrono::offset::Utc::now(),
        };

        let _ = create_message(&connection, chat.id, &message);
        let mut query = connection.prepare(
            "SELECT MessageId, Role, MessageContent, CreatedAt FROM Message WHERE MessageId = 1",
        )?;

        let mut query_result = query.query_map([], |row| {
            Ok(Message {
                id: row.get("MessageId")?,
                role: row.get("Role")?,
                content: row.get("MessageContent")?,
                created_at: row.get("CreatedAt")?,
            })
        })?;
        if let Ok(row) = query_result.next().unwrap() {
            let id = row.id;
            let role = row.role;
            let content = row.content;
            let _created_at = row.created_at;
            assert_eq!(id, 1);
            assert_eq!(role, "User".to_string());
            assert_eq!(content, "This is a test message".to_string());
        } else {
            panic!("No user found");
        }
        Ok(())
    }

    #[test]
    fn test_create_chat() -> Result<()> {
        let connection = setup_db_in_memory()?;
        let title = "test chat".to_string();
        let chat_id = create_chat(&title, &connection);
        let mut query = connection.prepare(
            format!(
                "SELECT ChatId, ChatTitle FROM Chat WHERE ChatId = {}",
                chat_id
            )
            .as_str(),
        )?;

        let mut query_result = query.query_map([], |row| {
            Ok(Chat {
                id: row.get("ChatId")?,
                title: row.get("ChatTitle")?,
                messages: Vec::new(),
            })
        })?;
        if let Ok(row) = query_result.next().unwrap() {
            let id = row.id;
            let title = row.title;
            assert_eq!(id, chat_id);
            assert_eq!(title, "test chat".to_string());
        } else {
            panic!("No user found");
        }
        Ok(())
    }

    #[test]
    fn test_delete_chat() -> Result<()> {
        let connection = setup_db_in_memory()?;
        let title = "test chat".to_string();
        let chat_id = create_chat(&title, &connection);
        let _ = delete_chat(&connection, chat_id);

        let mut query = connection.prepare("SELECT ChatId, ChatTitle FROM Chat")?;
        let mut query_result = query.query_map([], |row| {
            Ok(Chat {
                id: row.get("ChatId")?,
                title: row.get("ChatTitle")?,
                messages: Vec::new(),
            })
        })?;
        assert!(!query_result.any(|chat| chat.unwrap().id == chat_id));
        Ok(())
    }

    #[test]
    fn test_get_chat_messages() -> Result<()> {
        let connection = setup_db_in_memory()?;
        let title = "test chat".to_string();
        let chat_id = create_chat(&title, &connection);
        for i in 1..4 {
            let message = Message {
                id: i,
                role: "User".to_string(),
                content: "This is a test message".to_string(),
                created_at: chrono::offset::Utc::now(),
            };

            let _ = create_message(&connection, chat_id, &message);
        }
        let messages_ids: Vec<u32> = get_chat_messages(&connection, chat_id, None)
            .unwrap()
            .iter()
            .map(|msg| msg.id)
            .collect();
        assert!([1, 2, 3].iter().all(|x| messages_ids.contains(x)));
        Ok(())
    }

    #[test]
    fn test_get_all_chats() -> Result<()> {
        let connection = setup_db_in_memory()?;
        for i in 1..4 {
            let title = format!("test chat {}", i).to_string();
            let _chat_id = create_chat(&title, &connection);
        }
        let chats_ids: Vec<u32> = get_all_chats(&connection)
            .unwrap()
            .iter()
            .map(|chat| chat.id)
            .collect();
        assert!([1, 2, 3].iter().all(|x| chats_ids.contains(x)));
        Ok(())
    }

    #[test]
    fn test_get_chat() -> Result<()> {
        let connection = setup_db_in_memory()?;
        let title = "test chat".to_string();
        let chat_id = create_chat(&title, &connection);
        let chat = get_chat(&connection, chat_id);
        assert_eq!(chat.unwrap().title, "test chat".to_string());
        Ok(())
    }
}
