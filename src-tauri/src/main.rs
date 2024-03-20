// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::{thread_rng, Rng};

const NUM_POSSIBLE_ANSWERS: i32 = 1;

fn alternate_string_case(string: &str) -> String {
    string
        .char_indices()
        .map(|(idx, c)| {
            if idx % 2 == 0 {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect()
}

#[tauri::command]
fn message_to_reply(message: &str) -> (i32, String) {
    let reply_id = thread_rng().gen_range(0..=NUM_POSSIBLE_ANSWERS);
    let reply = match reply_id {
        0 => "Pong!".to_string(),
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
