pub mod client;

use derive_more::From;
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::generic::{BooruPost, Rating};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DanbooruPost {
    pub id: u32,
    pub created_at: String,
    pub updated_at: String,
    pub uploader_id: u32,
    pub approver_id: u32,
    pub tag_string: String,
    pub tag_string_general: String,
    pub tag_string_artist: String,
    pub tag_string_copyright: String,
    pub tag_string_character: String,
    pub tag_string_meta: String,
    pub rating: Option<DanbooruRating>,
    pub parent_id: Option<u32>,
    pub pixiv_id: Option<u32>,
    pub source: String,
    pub md5: Option<String>,
    pub file_url: String,
    pub large_file_url: String,
    pub preview_file_url: String,
    pub file_ext: String,
    pub file_size: u32,
    pub image_width: u32,
    pub image_height: u32,
    pub score: i32,
    pub up_score: i32,
    pub down_score: i32,
    pub fav_count: u32,
    pub tag_count_general: u32,
    pub tag_count_artist: u32,
    pub tag_count_copyright: u32,
    pub tag_count_character: u32,
    pub tag_count_meta: u32,
    pub last_comment_bumped_at: Option<String>,
    pub last_noted_at: Option<String>,
    pub has_large: bool,
    pub has_children: bool,
    pub has_visible_children: bool,
    pub has_active_children: bool,
    pub is_banned: bool,
    pub is_deleted: bool,
    pub is_flagged: bool,
    pub is_pending: bool,
    pub bit_flags: u32,
}

/// Post's rating. Check the [Danbooru's ratings wiki](https://danbooru.donmai.us/wiki_pages/howto:rate)
#[derive(Serialize, Deserialize, Debug, Clone, Display, From)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum DanbooruRating {
    #[serde(rename = "e")]
    Explicit,
    #[serde(rename = "q")]
    Questionable,
    #[serde(rename = "s")]
    Sensitive,
    #[serde(rename = "g")]
    General,
}

impl From<Rating> for DanbooruRating {
    fn from(value: Rating) -> Self {
        match value {
            Rating::Explicit => Self::Explicit,
            Rating::Questionable => Self::Questionable,
            Rating::Safe => Self::General,
            Rating::Sensitive => Self::Sensitive,
            Rating::General => Self::General,
        }
    }
}

impl From<DanbooruRating> for Rating {
    fn from(value: DanbooruRating) -> Self {
        match value {
            DanbooruRating::Explicit => Rating::Explicit,
            DanbooruRating::Questionable => Rating::Questionable,
            DanbooruRating::Sensitive => Rating::Sensitive,
            DanbooruRating::General => Rating::General,
        }
    }
}

impl From<DanbooruPost> for BooruPost {
    fn from(value: DanbooruPost) -> Self {
        Self {
            id: value.id,
            created_at: value.created_at.into(),
            score: value.score.into(),
            width: value.image_width,
            height: value.image_height,
            md5: value.md5,
            file_url: value.file_url.into(),
            tags: value.tag_string,
            image: None,
            source: value.source.into(),
            rating: value.rating.unwrap_or(DanbooruRating::Sensitive).into(),
        }
    }
}
