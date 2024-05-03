use crate::{
    gemini::{get_chat_reponse, lib::Content},
    requests::{handle_json_requests, handle_text_requests},
};
use urlencoding::encode;

const ERROR_MESSAGE: &str = "Oopsie";

fn handle_request_errors(data: Option<String>, error: Option<String>) -> String {
    if let Some(error) = error {
        eprintln!("{error}")
    }
    match data {
        Some(data) => data,
        None => ERROR_MESSAGE.to_string(),
    }
}

pub fn get_affirmation() -> String {
    let (data, error) = handle_json_requests("https://www.affirmations.dev/", "affirmation");
    handle_request_errors(data, error)
}

pub fn get_joke() -> String {
    let (data, error) = handle_text_requests("https://icanhazdadjoke.com/", &vec![]);
    handle_request_errors(data, error)
}

pub fn get_weather(city: &str) -> String {
    let encoded_city = encode(city).into_owned();
    let (data, error) = handle_text_requests(
        format!("https://wttr.in/{encoded_city}").as_str(),
        &vec![("format".to_string(), "3".to_string())],
    );
    handle_request_errors(data, error)
}

pub fn chat_gemini(apikey: &str, history: &[Content]) -> String {
    let (data, error) = get_chat_reponse(apikey, history);
    handle_request_errors(data, error)
}

#[cfg(test)]
mod test {
    use super::{get_joke, get_weather};
    use crate::api::{get_affirmation, ERROR_MESSAGE};

    #[test]
    fn test_get_weather() {
        let data = get_weather("");
        assert_ne!(data, ERROR_MESSAGE)
    }

    #[test]
    fn test_get_joke() {
        let data = get_joke();
        assert_ne!(data, ERROR_MESSAGE)
    }

    #[test]
    fn test_get_affirmation() {
        let data = get_affirmation();
        assert_ne!(data, ERROR_MESSAGE)
    }
}
