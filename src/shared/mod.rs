use self::client::{ClientInformation, ClientTypes};
use derive_is_enum_variant::is_enum_variant;
use itertools::Itertools;
use std::fmt::Display;
use strum::Display;
use thiserror::Error;

pub mod client;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Requested booru client with way too many tags.")]
    TooManyTags,
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

#[derive(is_enum_variant, Clone)]
pub enum Tag<T: ClientTypes> {
    Plain(String),
    Blacklist(String),
    Rating(T::Rating),
    Sort(Sort),
}

impl<T: ClientInformation + ClientTypes> Display for Tag<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::Plain(tag) => write!(f, "{}", tag),
            Tag::Blacklist(tag) => write!(f, "-{}", tag),
            Tag::Rating(tag) => write!(f, "rating:{}", tag),
            Tag::Sort(by) => write!(f, "{}:{}", T::SORT, by),
        }
    }
}

pub struct Tags<T: ClientTypes>(pub Vec<Tag<T>>);

impl<T: ClientTypes + ClientInformation> Tags<T> {
    pub fn unpack(&self) -> String {
        self.0
            .iter()
            .map(ToString::to_string)
            .collect_vec()
            .join(" ")
    }
}
