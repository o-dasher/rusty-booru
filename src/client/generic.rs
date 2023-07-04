use std::{convert::Infallible, fmt::Display, marker::PhantomData};

use derive_is_enum_variant::is_enum_variant;
use derive_more::From;
use itertools::Itertools;
use strum::Display;

use crate::{
    model::{danbooru::DanbooruRating, gelbooru::GelbooruRating, safebooru::SafebooruRating},
    Client,
};

#[derive(From, Display)]
pub enum Rating {
    Danbooru(DanbooruRating),
    Gelbooru(GelbooruRating),
    Safebooru(SafebooruRating),
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
pub enum Tag<'a, R: Into<Rating> + Display, T: Client<'a, R>> {
    Plain(String),
    Blacklist(String),
    Rating(R),
    Sort(Sort),
    #[is_enum_variant(skip)]
    _Marker(Infallible, &'a PhantomData<T>),
}

impl<'a, R: Into<Rating> + Display, T: Client<'a, R>> Display for Tag<'a, R, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::Plain(tag) => write!(f, "{}", tag),
            Tag::Blacklist(tag) => write!(f, "-{}", tag),
            Tag::Rating(tag) => write!(f, "rating:{}", tag),
            Tag::Sort(by) => write!(f, "{}:{}", T::SORT, by),
        }
    }
}

pub struct Tags<'a, R: Into<Rating> + Display, T: Client<'a, R>>(pub Vec<Tag<'a, R, T>>);

impl<'a, R: Into<Rating> + Display, T: Client<'a, R>> Tags<'a, R, T> {
    pub fn unpack(&self) -> String {
        self.0
            .iter()
            .map(ToString::to_string)
            .collect_vec()
            .join(" ")
    }
}
