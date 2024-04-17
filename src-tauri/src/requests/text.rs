use super::handle_text_requests;

pub fn get_affirmation() -> String {
    let (data, error) = handle_text_requests("https://www.affirmations.dev/", "affirmation");
    if let Some(error) = error {
        eprintln!("{error}")
    }
    match data {
        Some(data) => data,
        None => "Oopsie".to_string(),
    }
}
