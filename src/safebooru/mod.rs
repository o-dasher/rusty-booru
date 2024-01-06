pub mod client;

use derive_more::From;

use serde::Deserialize;
use strum::Display;

use crate::{
    generic::{BooruPost, Rating},
    shared::client::ClientInformation,
};

use self::client::SafebooruClient;

#[derive(Deserialize, Debug, Clone)]
pub struct SafebooruPost {
    pub id: u32,
    pub score: Option<u32>,
    /// This can be `null` for really recent posts
    pub height: u32,
    pub width: u32,
    pub hash: String,
    pub tags: String,
    pub image: String,
    /// This is basically equivalent to `updated_at` in a Danbooru post. Except
    /// that it's provided as a UNIX timestamp. Safebooru provides no `created_at`
    /// field.
    pub change: u32,
    pub rating: SafebooruRating,
}

#[derive(Deserialize, Debug, Clone, Display, From)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SafebooruRating {
    Safe,
    General,
    // Yes there are explicit and questionable posts. Though you only need to care
    // about them if you're querying for deleted content.
    Questionable,
    Explicit,
}

impl From<Rating> for SafebooruRating {
    fn from(value: Rating) -> Self {
        match value {
            Rating::Explicit => SafebooruRating::Explicit,
            Rating::Questionable => SafebooruRating::Questionable,
            Rating::Safe => SafebooruRating::Safe,
            Rating::Sensitive => SafebooruRating::Questionable,
            Rating::General => SafebooruRating::General,
        }
    }
}

impl From<SafebooruRating> for Rating {
    fn from(value: SafebooruRating) -> Self {
        match value {
            SafebooruRating::Safe => Rating::Safe,
            SafebooruRating::General => Rating::General,
            SafebooruRating::Questionable => Rating::Questionable,
            SafebooruRating::Explicit => Rating::Explicit,
        }
    }
}

impl From<SafebooruPost> for BooruPost {
    fn from(post: SafebooruPost) -> Self {
        Self {
            id: post.id,
            created_at: None,
            score: post.score.unwrap_or_default().into(),
            width: post.width,
            height: post.height,
            md5: None,
            file_url: format!("{}/images/4491/{}", SafebooruClient::URL, post.image),
            tags: post.tags,
            image: post.image.into(),
            source: None,
            rating: post.rating.into(),
        }
    }
}
