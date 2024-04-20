// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod gemini;
mod requests;
mod silly;

use dotenv::dotenv;
use gemini::lib::{Content, Part};
use rand::{thread_rng, Rng};
use requests::text::{chat_gemini, get_affirmation, get_joke};
use silly::alternate_string_case;
use std::env;

// TODO there's probably a better way to do that
static mut HISTORY: Vec<Vec<Content>> = vec![];

const NUM_POSSIBLE_ANSWERS: i32 = 3;

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
        _ => alternate_string_case(message),
    };
    (reply_id, reply)
}

fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![message_to_reply])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
