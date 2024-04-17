pub mod text;

use std::collections::HashMap;

pub fn handle_text_requests(url: &str, key: &str) -> (Option<String>, Option<String>) {
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
    use super::handle_text_requests;

    #[test]
    fn test_fail_request_should_contain_error_and_no_data() {
        let (data, err) = handle_text_requests("", "");

        assert!(data.is_none());
        assert!(err.is_some());

        let err = err.unwrap();
        assert!(err.contains("making a request"));
    }

    #[test]
    fn test_fail_conversion_should_contain_error_and_no_data() {
        let (data, err) = handle_text_requests("https://youtube.com", "");

        assert!(data.is_none());
        assert!(err.is_some());

        let err = err.unwrap();
        assert!(err.contains("processing the response"));
    }

    #[test]
    fn test_fail_accessing_data_should_contain_error_and_no_data() {
        let (data, err) = handle_text_requests("https://www.affirmations.dev/", "");

        assert!(data.is_none());
        assert!(err.is_some());

        let err = err.unwrap();
        assert!(err.contains("accessing response data"));
    }
}
