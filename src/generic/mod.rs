pub mod client;

use strum::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Requested booru client with way too many tags.")]
    TooManyTags,
}

#[derive(Display, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum Rating {
    Explicit,
    Questionable,
    Safe,
    Sensitive,
    General,
}

#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Sort {
    Id,
    Score,
    Rating,
    User,
    Height,
    Width,
    Source,
    Updated,
    Random,
}


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
