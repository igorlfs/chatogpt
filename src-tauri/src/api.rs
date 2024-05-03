use crate::{
    gemini::{get_chat_reponse, lib::Content},
    requests::{handle_json_requests, handle_text_requests, RequestOptions},
};
use urlencoding::encode;

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
    let (data, error) = handle_json_requests("https://www.affirmations.dev/", "affirmation");
    handle_request_errors(data, error)
}

pub fn get_joke() -> String {
    let options = RequestOptions {
        text: true,
        ..Default::default()
    };
    let (data, error) = handle_text_requests("https://icanhazdadjoke.com/", &options);
    handle_request_errors(data, error)
}

pub fn get_weather(city: &str) -> String {
    let encoded_city = encode(city).into_owned();
    let options = RequestOptions {
        query: vec![("format".to_string(), "3".to_string())],
        ..Default::default()
    };
    let (data, error) =
        handle_text_requests(format!("https://wttr.in/{encoded_city}").as_str(), &options);
    handle_request_errors(data, error)
}

pub fn chat_gemini(apikey: &str, history: &[Content]) -> String {
    let (data, error) = get_chat_reponse(apikey, history);
    handle_request_errors(data, error)
}
