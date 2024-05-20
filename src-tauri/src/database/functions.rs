use rusqlite::Connection;
use std::error::Error;

use super::model::{Chat, Message};

pub fn get_chat(connection: &Connection, chat_id: u32) -> Result<Chat, Box<dyn Error>> {
    let mut query = connection
        .prepare(format!("SELECT * FROM Chat WHERE ChatId = {chat_id} ORDER BY").as_str())?;

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
