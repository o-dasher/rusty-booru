use derive_more::From;

use serde::Deserialize;
use strum::Display;

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
