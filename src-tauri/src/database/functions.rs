use rusqlite::Connection;
use std::error::Error;

use super::model::{Chat, Message};

pub fn get_chat(connection: &Connection, chat_id: u32) -> Result<Chat, Box<dyn Error>> {
    let mut query =
        connection.prepare(format!("SELECT * FROM Chat WHERE ChatId = {}", chat_id).as_str())?;

    let query_result = query.quey_map([], |row| {
        Ok(Chat {
            id: row.get("ChatId")?,
            title: row.get("ChatTitle")?,
            created_at: row.get("CreatedAt")?,
            updated_at: row.get("UpdatedAt")?,
        })
    })?;

    let mut chat = query_result.next().unwrap().unwrap();

    chat.messages = get_chat_messages(connection, chat.id).unwrap();

    Ok(chat)
}

pub fn get_chat_messages(
    connection: &Connection,
    chat_id: u32,
) -> Result<Vec<Message>, Box<dyn Error>> {
    let mut query =
        connection.prepare(format!("SELECT * FROM Message WHERE ChatId = {}", chat_id).as_str())?;

    let query_result = query.quey_map([], |row| {
        Ok(Message {
            id: row.get("MessageId")?,
            content: row.get("ChatTitle")?,
            created_at: row.get("CreatedAt")?,
        })
    })?;

    Ok(query_result.map(|opt| opt.unwrap()).collect())
}
