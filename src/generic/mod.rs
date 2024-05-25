pub mod client;

use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Display, Debug, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum Rating {
    Explicit,
    Questionable,
    Safe,
    Sensitive,
    General,
}

#[derive(Debug, Clone)]
pub struct BooruPost {
    pub id: u32,
    pub created_at: Option<String>,
    pub score: i64,
    pub width: u32,
    pub height: u32,
    pub md5: Option<String>,
    pub file_url: Option<String>,
    pub tags: String,
    pub image: Option<String>,
    pub source: Option<String>,
    pub rating: Rating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoCompleteItem {
    pub value: String,
    pub label: String,
}
