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
use database::{
    functions::{
        create_chat, create_message, delete_chat, get_all_chats, get_chat, get_chat_messages,
    },
    model::{Chat, Message},
};
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
fn get_reply_command(db: State<Database>, message: &str, chat_id: u32) -> String {
    let apikey = env::var("APIKEY");
    let connection = db.connection.lock().unwrap();

    // TODO we shouldn't always default to gemini
    if apikey.is_ok() {
        let apikey = apikey.unwrap();
        let chat_messages =
            get_chat_messages(&connection, chat_id, Some(MAX_CONTEXT_LENGTH)).unwrap();
        let chat_content = chat_messages
            .iter()
            .rev()
            .map(|msg| Content {
                role: msg.role.to_string(),
                parts: vec![Part::Text(msg.content.clone())],
            })
            .collect();
        let reply = chat_gemini(&apikey, chat_content);
        return reply;
    }

    let reply_id = thread_rng().gen_range(0..=NUM_POSSIBLE_ANSWERS);
    match reply_id {
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
    }
}

#[tauri::command]
fn create_chat_command(db: State<Database>, title: &str) -> u32 {
    let connection = db.connection.lock().unwrap();
    create_chat(title, &connection)
}

#[tauri::command]
fn get_chat_command(db: State<Database>, chat_id: u32) -> Chat {
    let connection = db.connection.lock().unwrap();
    get_chat(&connection, chat_id).unwrap()
}

#[tauri::command]
fn delete_chat_command(db: State<Database>, chat_id: u32) {
    let connection = db.connection.lock().unwrap();
    delete_chat(&connection, chat_id).unwrap();
}

#[tauri::command]
fn get_all_chats_command(db: State<Database>) -> Vec<Chat> {
    let connection = db.connection.lock().unwrap();
    get_all_chats(&connection).unwrap()
}

#[tauri::command]
fn create_message_command(db: State<Database>, chat_id: u32, message: Message) {
    let connection = db.connection.lock().unwrap();
    create_message(&connection, chat_id, &message).unwrap();
}

#[cfg(not(tarpaulin_include))]
fn main() {
    dotenv().ok();
    let connection = database::connect();
    tauri::Builder::default()
        .manage(Database {
            connection: Mutex::new(connection),
        })
        .invoke_handler(tauri::generate_handler![
            get_reply_command,
            create_chat_command,
            get_chat_command,
            delete_chat_command,
            get_all_chats_command,
            create_message_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
