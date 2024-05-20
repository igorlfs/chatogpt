use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

fn default_id() -> u32 {
    0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(default = "default_id")]
    pub id: u32,
    pub role: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    #[serde(default = "default_id")]
    pub id: u32,
    pub title: String,
    pub messages: Vec<Message>,
}
