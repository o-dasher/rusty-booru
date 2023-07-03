use derive_more::From;

use crate::model::{
    danbooru::DanbooruRating, gelbooru::GelbooruRating, safebooru::SafebooruRating,
};
use std::fmt;

#[derive(From)]
pub enum Rating {
    Danbooru(DanbooruRating),
    Gelbooru(GelbooruRating),
    Safebooru(SafebooruRating),
}

#[derive(Debug, Clone)]
pub enum Sort {
    Id,
    Score,
    Rating,
    User,
    Height,
    Width,
    Source,
    Updated,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase_tag}")
    }
}
