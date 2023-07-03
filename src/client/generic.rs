use derive_more::From;
use strum::Display;

use crate::{
    model::{danbooru::DanbooruRating, gelbooru::GelbooruRating, safebooru::SafebooruRating},
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
}
