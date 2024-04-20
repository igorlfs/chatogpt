use super::{handle_text_requests_from_json, handle_text_requests_from_text};
use crate::gemini::{get_chat_reponse, lib::Content};

// TODO things are getting out of hand

pub fn get_affirmation() -> String {
    let (data, error) =
        handle_text_requests_from_json("https://www.affirmations.dev/", "affirmation");
    if let Some(error) = error {
        eprintln!("{error}")
    }
    match data {
        Some(data) => data,
        None => "Oopsie".to_string(),
    }
}

pub fn get_joke() -> String {
    let (data, error) = handle_text_requests_from_text("https://icanhazdadjoke.com/");
    if let Some(error) = error {
        eprintln!("{error}")
    }
    match data {
        Some(data) => data,
        None => "Oopsie".to_string(),
    }
}

pub fn chat_gemini(apikey: &str, history: &[Content]) -> String {
    let (data, error) = get_chat_reponse(apikey, history);
    if let Some(error) = error {
        eprintln!("{error}")
    }
    match data {
        Some(data) => data,
        None => "Oopsie".to_string(),
    }
}
