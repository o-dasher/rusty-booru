use std::fmt::Display;

use derive_is_enum_variant::is_enum_variant;
use itertools::Itertools;
use strum::Display;
use thiserror::Error;

use super::client::ClientInformation;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Requested booru client with way too many tags.")]
    TooManyTags,
}

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

#[derive(is_enum_variant)]
pub enum Tag<T: ClientInformation> {
    Plain(String),
    Blacklist(String),
    Rating(T::Rating),
    Sort(Sort),
}

impl<T: ClientInformation> Display for Tag<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::Plain(tag) => write!(f, "{}", tag),
            Tag::Blacklist(tag) => write!(f, "-{}", tag),
            Tag::Rating(tag) => write!(f, "rating:{}", tag),
            Tag::Sort(by) => write!(f, "{}:{}", T::SORT, by),
        }
    }
}

pub struct Tags<T: ClientInformation>(pub Vec<Tag<T>>);

impl<T: ClientInformation> Tags<T> {
    pub fn unpack(&self) -> String {
        self.0
            .iter()
            .map(ToString::to_string)
            .collect_vec()
            .join(" ")
    }
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
