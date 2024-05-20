pub mod lib;

use lib::{
    Content, GenerateContentRequest, GenerateContentResponse, GenerationConfig, Part,
};

pub fn get_chat_reponse(apikey: &str, history: Vec<Content>) -> (Option<String>, Option<String>) {
    let payload = GenerateContentRequest {
        contents: history,
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
