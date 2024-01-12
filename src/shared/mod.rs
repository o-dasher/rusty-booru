use self::client::{ClientInformation, ClientTypes};
use derive_is_enum_variant::is_enum_variant;
use itertools::Itertools;
use std::fmt::Display;
use strum::Display;

pub mod client;

#[derive(derive_more::From, Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(reqwest::Error),

    #[error("Searched for way too many tags on a client that does not support it.")]
    TagLimitError,

    #[error("Something rather fishy happened while using rusty-booru.")]
    Unexpected,
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

#[derive(Clone)]
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
