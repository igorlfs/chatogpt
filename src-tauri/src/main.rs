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
use dotenv::dotenv;
use encryption::{caesar_cipher, vigenere_cipher};
use gemini::lib::{Content, Part};
use password::is_password_secure;
use rand::{thread_rng, Rng};
use std::env;
use strings::{alternate_string_case, is_string_ordered, match_email_address};

// TODO there's probably a better way to do that
static mut HISTORY: Vec<Vec<Content>> = vec![];

const NUM_POSSIBLE_ANSWERS: i32 = 9;

#[tauri::command]
fn message_to_reply(message: &str, thread_id: usize) -> (i32, String) {
    unsafe {
        if HISTORY.len() <= thread_id {
            HISTORY.resize(thread_id + 1, vec![]);
        }
        HISTORY[thread_id].push(Content {
            role: "user".to_string(),
            parts: vec![Part::Text(message.to_string())],
        });
    }
    let apikey = env::var("APIKEY");

    // TODO we shouldn't always default to gemini
    if apikey.is_ok() {
        let apikey = apikey.unwrap();
        let reply;
        unsafe {
            reply = chat_gemini(&apikey, &HISTORY[thread_id]);
            HISTORY[thread_id].push(Content {
                role: "model".to_string(),
                parts: vec![Part::Text(reply.clone())],
            });
        }
        return (-1, reply);
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
    (reply_id, reply)
}

#[cfg(not(tarpaulin_include))]
fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![message_to_reply])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
