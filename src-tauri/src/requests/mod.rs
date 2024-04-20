pub mod gemini;
pub mod text;

use reqwest::header::{HeaderValue, ACCEPT};
use std::collections::HashMap;

use self::gemini::{
    Content, GenerateContentRequest, GenerateContentResponse, GenerationConfig, Part,
};

pub fn handle_text_requests_from_text(url: &str) -> (Option<String>, Option<String>) {
    let client = reqwest::blocking::Client::new();
    match client
        .get(url)
        .header(ACCEPT, HeaderValue::from_static("text/plain"))
        .send()
    {
        Ok(response) => (Some(response.text().unwrap()), None),
        Err(e) => (
            None,
            Some(format!("An error occurred when making a request: {}", e).to_string()),
        ),
    }
}

pub fn handle_text_requests_from_json(url: &str, key: &str) -> (Option<String>, Option<String>) {
    match reqwest::blocking::get(url) {
        Ok(response) => match response.json::<HashMap<String, String>>() {
            Ok(object) => match object.get(key) {
                Some(data) => (Some(data.to_string()), None),
                None => (
                    None,
                    Some("An error occurred when accessing response data".to_string()),
                ),
            },
            Err(e) => (
                None,
                Some(format!("An error occurred while processing the response: {}", e).to_string()),
            ),
        },
        Err(e) => (
            None,
            Some(format!("An error occurred when making a request: {}", e).to_string()),
        ),
    }
}

pub fn get_gemini_response(apikey: &str, history: &[Content]) -> (Option<String>, Option<String>) {
    let payload = GenerateContentRequest {
        contents: history.to_vec(),
        generation_config: Some(GenerationConfig {
            max_output_tokens: Some(1000),
            temperature: Some(0.4),
            top_p: Some(1.0),
            top_k: Some(32),
            ..Default::default()
        }),
        tools: None,
    };
    let client = reqwest::blocking::Client::new();
    match client.post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={apikey}")).json(&payload).send() {
        Ok(response) => match response.json::<GenerateContentResponse>() {
            Ok(gemini_response) => {
                let aux = &gemini_response.candidates[0].content.parts[0];
                match aux { 
                    Part::Text(text) => (Some(text.clone()), None), 
                    _ => (None, Some("Unexpected response from Gemini".to_string()))
                }
            },
            Err(e) => (
                None,
                Some(format!("An error occurred while processing the response: {}", e).to_string()),
            ),
        },
        Err(e) => (
            None,
            Some(format!("An error occurred when making a request: {}", e).to_string()),
        ),
    }
}

#[cfg(test)]
mod test {
    use super::{handle_text_requests_from_json, handle_text_requests_from_text};

    #[test]
    fn test_json_fail_request_should_contain_error_and_no_data() {
        let (data, err) = handle_text_requests_from_json("", "");

        assert!(data.is_none());
        assert!(err.is_some());

        let err = err.unwrap();
        assert!(err.contains("making a request"));
    }

    #[test]
    fn test_json_fail_conversion_should_contain_error_and_no_data() {
        let (data, err) = handle_text_requests_from_json("https://youtube.com", "");

        assert!(data.is_none());
        assert!(err.is_some());

        let err = err.unwrap();
        assert!(err.contains("processing the response"));
    }

    #[test]
    fn test_json_fail_accessing_data_should_contain_error_and_no_data() {
        let (data, err) = handle_text_requests_from_json("https://www.affirmations.dev/", "");

        assert!(data.is_none());
        assert!(err.is_some());

        let err = err.unwrap();
        assert!(err.contains("accessing response data"));
    }

    #[test]
    fn test_text_fail_request_should_contain_error_and_no_data() {
        let (data, err) = handle_text_requests_from_text("");

        assert!(data.is_none());
        assert!(err.is_some());

        let err = err.unwrap();
        assert!(err.contains("making a request"));
    }
}
