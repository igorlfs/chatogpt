// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod requests;
mod silly;

use rand::{thread_rng, Rng};
use requests::text::{get_affirmation, get_joke};
use silly::alternate_string_case;

const NUM_POSSIBLE_ANSWERS: i32 = 3;

#[tauri::command]
fn message_to_reply(message: &str) -> (i32, String) {
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
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![message_to_reply])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
