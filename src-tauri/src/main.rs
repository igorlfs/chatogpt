// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod database;
mod encryption;
mod gemini;
mod password;
mod requests;
mod strings;

use api::{chat_gemini, get_affirmation, get_joke, get_weather};
use chrono::Local;
use database::model::Message;
use dotenv::dotenv;
use encryption::{caesar_cipher, vigenere_cipher};
use gemini::lib::{Content, Part};
use password::is_password_secure;
use rand::{thread_rng, Rng};
use rusqlite::Connection;
use std::env;
use std::sync::Mutex;
use strings::{alternate_string_case, is_string_ordered, match_email_address};
use tauri::State;

const NUM_POSSIBLE_ANSWERS: i32 = 9;
const MAX_CONTEXT_LENGTH: u32 = 5;

struct Database {
    connection: Mutex<Connection>,
}

#[tauri::command]
fn message_to_reply(db: State<Database>, message: &str, chat_id: u32) -> String {
    let apikey = env::var("APIKEY");
    let connection = db.connection.lock().unwrap();
    let mut new_message = Message {
        id: 1, // deus que me perdoe por essa gambiarra
        created_at: Local::now().into(),
        content: "".to_string(),
        role: "Chato".to_string(),
    };

    // TODO we shouldn't always default to gemini
    if apikey.is_ok() {
        let apikey = apikey.unwrap();
        let chat_messages =
            database::functions::get_chat_messages(&connection, chat_id, Some(MAX_CONTEXT_LENGTH))
                .unwrap();
        let chat_content = chat_messages
            .iter()
            .rev()
            .map(|msg| Content {
                role: msg.role.to_string(),
                parts: vec![Part::Text(msg.content.clone())],
            })
            .collect();
        let reply = chat_gemini(&apikey, chat_content);
        new_message.content = reply.clone();
        let _ = database::functions::create_message(&connection, chat_id, &new_message);
        return reply;
    }

    let reply_id = thread_rng().gen_range(0..=NUM_POSSIBLE_ANSWERS);
    let reply = match reply_id {
        0 => "Pong!".to_string(),
        1 => get_affirmation(),
        2 => get_joke(),
        3 => get_weather(env::var("CITY").unwrap_or_default().as_str()),
        4 => caesar_cipher(message, 13),        // ROT 13
        5 => vigenere_cipher(message, "syrax"), // Fire && Blood
        6 => match_email_address(message),
        7 => is_password_secure(message),
        8 => is_string_ordered(message),
        _ => alternate_string_case(message),
    };
    new_message.content = reply.clone();
    let _ = database::functions::create_message(&connection, chat_id, &new_message);
    reply
}

#[tauri::command]
fn create_chat_command(db: State<Database>, chat_json: String) -> Result<String, String> {
    let chat = serde_json::from_str(&chat_json).unwrap();
    let connection = db.connection.lock().unwrap();
    let created_chat = database::functions::create_chat(&chat, &connection).unwrap();
    let created_chat_json = serde_json::to_string(&created_chat).unwrap();
    Ok(created_chat_json)
}

#[tauri::command]
fn get_chat_command(db: State<Database>, chat_id: u32) -> Result<String, String> {
    let connection = db.connection.lock().unwrap();
    let chat = database::functions::get_chat(&connection, chat_id).unwrap();
    let chat_json = serde_json::to_string(&chat).unwrap();
    Ok(chat_json)
}

#[tauri::command]
fn delete_chat_command(db: State<Database>, chat_id: u32) -> Result<(), String> {
    let connection = db.connection.lock().unwrap();
    let _ = database::functions::delete_chat(&connection, chat_id);
    Ok(())
}

#[tauri::command]
fn get_all_chats_command(db: State<Database>) -> Result<Vec<String>, String> {
    let connection = db.connection.lock().unwrap();
    let all_chats = database::functions::get_all_chats(&connection).unwrap();
    let all_chats_json = all_chats
        .iter()
        .map(|chat| serde_json::to_string(&chat).unwrap())
        .collect();
    Ok(all_chats_json)
}

#[tauri::command]
fn create_message(
    db: State<Database>,
    chat_id: u32,
    message_json: String,
) -> Result<String, String> {
    let connection = db.connection.lock().unwrap();
    // Aqui a role da mensagem deve ser de User
    let message = serde_json::from_str(&message_json).unwrap();
    let created_message =
        database::functions::create_message(&connection, chat_id, &message).unwrap();
    let created_message_json = serde_json::to_string(&created_message).unwrap();
    Ok(created_message_json)
}

#[cfg(not(tarpaulin_include))]
fn main() {
    dotenv().ok();
    let connection = database::connect().unwrap();
    tauri::Builder::default()
        .manage(Database {
            connection: Mutex::new(connection),
        })
        .invoke_handler(tauri::generate_handler![
            message_to_reply,
            create_chat_command,
            get_chat_command,
            delete_chat_command,
            get_all_chats_command,
            create_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
