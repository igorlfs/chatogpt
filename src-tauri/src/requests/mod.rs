pub mod text;

use reqwest::header::{HeaderValue, ACCEPT};
use std::collections::HashMap;

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
