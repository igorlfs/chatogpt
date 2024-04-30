use crate::{
    gemini::{get_chat_reponse, lib::Content},
    requests::{handle_text_requests_from_json, handle_text_requests_from_text},
};

fn handle_request_errors(data: Option<String>, error: Option<String>) -> String {
    if let Some(error) = error {
        eprintln!("{error}")
    }
    match data {
        Some(data) => data,
        None => "Oopsie".to_string(),
    }
}

pub fn get_affirmation() -> String {
    let (data, error) =
        handle_text_requests_from_json("https://www.affirmations.dev/", "affirmation");
    handle_request_errors(data, error)
}

pub fn get_joke() -> String {
    let (data, error) = handle_text_requests_from_text("https://icanhazdadjoke.com/");
    handle_request_errors(data, error)
}

pub fn chat_gemini(apikey: &str, history: &[Content]) -> String {
    let (data, error) = get_chat_reponse(apikey, history);
    handle_request_errors(data, error)
}
