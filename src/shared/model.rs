use std::fmt::Display;

use derive_is_enum_variant::is_enum_variant;
use itertools::Itertools;
use strum::Display;

use super::client::ClientInformation;

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
