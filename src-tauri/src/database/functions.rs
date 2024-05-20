use rusqlite::Connection;
use std::error::Error;

use super::model::{Chat, Message};

pub fn get_chat(connection: &Connection, chat_id: u32) -> Result<Chat, Box<dyn Error>> {
    let mut query = connection
        .prepare(format!("SELECT * FROM Chat WHERE ChatId = {} ORDER BY", chat_id).as_str())?;

    let mut query_result = query.query_map([], |row| {
        Ok(Chat {
            id: row.get("ChatId")?,
            title: row.get("ChatTitle")?,
            created_at: row.get("CreatedAt")?,
            updated_at: row.get("UpdatedAt")?,
            messages: vec![],
        })
    })?;

    let mut chat = query_result.next().unwrap().unwrap();

    chat.messages = get_chat_messages(connection, chat.id, None).unwrap();

    Ok(chat)
}

pub fn create_chat(chat: &Chat, connection: &Connection) -> Result<Chat, Box<dyn Error>> {
    let mut query = connection.prepare(
        format!(
            "INSERT INTO Chat (ChatTitle, CreatedAt, UpdatedAt) VALUES ('{}', {}, {})",
            chat.title,
            chat.created_at.format("%+"),
            chat.updated_at.format("%+"),
        )
        .as_str(),
    )?;
    let mut new_chat = chat.clone();
    new_chat.id = query.insert([])? as u32;
    Ok(new_chat)
}

pub fn delete_chat(connection: &Connection, chat_id: u32) -> Result<(), Box<dyn Error>> {
    let mut query =
        connection.prepare(format!("DELETE FROM Message WHERE ChatId = {}", chat_id).as_str())?;
    query.execute([])?;
    query = connection.prepare(format!("DELETE FROM Chat WHERE ChatId = {}", chat_id).as_str())?;
    query.execute([])?;
    Ok(())
}

pub fn get_all_chats(connection: &Connection) -> Result<Vec<Chat>, Box<dyn Error>> {
    let mut query = connection.prepare("SELECT * FROM Chat")?;
    let query_result = query.query_map([], |row| {
        Ok(Chat {
            id: row.get("ChatId")?,
            title: row.get("ChatTitle")?,
            created_at: row.get("CreatedAt")?,
            updated_at: row.get("UpdatedAt")?,
            messages: vec![],
        })
    })?;
    Ok(query_result.map(|opt| opt.unwrap()).collect())
}

pub fn get_chat_messages(
    connection: &Connection,
    chat_id: u32,
    limit: Option<u32>,
) -> Result<Vec<Message>, Box<dyn Error>> {
    let mut query = if let Some(limit) = limit {
        connection.prepare(
            format!(
                "SELECT * FROM Message WHERE ChatId = {} ORDER BY CreatedAt DESC LIMIT {}",
                chat_id, limit
            )
            .as_str(),
        )?
    } else {
        connection.prepare(
            format!(
                "SELECT * FROM Message WHERE ChatId = {} ORDER BY CreatedAt DESC",
                chat_id
            )
            .as_str(),
        )?
    };

    let query_result = query.query_map([], |row| {
        Ok(Message {
            id: row.get("MessageId")?,
            content: row.get("ChatTitle")?,
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
) -> Result<Message, Box<dyn Error>> {
    let mut query = connection.prepare(
        format!(
            "INSERT INTO Message (MessageContent, Role, ChatId, CreatedAt) VALUES ('{}', '{}',{}, {})",
            message.content,
            message.role, 
            chat_id,
            message.created_at.format("%+"),
        )
        .as_str(),
    )?;
    let mut new_message = message.clone();
    new_message.id = query.insert([])? as u32;
    Ok(new_message)
}
