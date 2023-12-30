use std::fmt::Display;

use super::model::{Rating, Sort, Tag, Tags, ValidationError};
use async_trait::async_trait;

pub struct ClientBuilder<T: ClientInformation> {
    pub client: reqwest::Client,
    pub tags: Tags<T>,
    pub limit: u32,
    pub url: String,
}

pub enum ValidationType<'a, T: ClientInformation> {
    Tags(&'a Tags<T>),
}

pub trait ClientInformation {
    const URL: &'static str;
    const SORT: &'static str;

    type Rating: From<Rating> + Display;
    type Post;
}

pub type QueryTuple = (&'static str, &'static str);
pub type QueryVec = Vec<(&'static str, &'static str)>;

pub enum BaseQuery {
    GelbooruLike,
}

impl BaseQuery {
    pub fn get(&self) -> QueryVec {
        match self {
            BaseQuery::GelbooruLike => vec![
                ("page", "dapi"),
                ("s", "post"),
                ("q", "index"),
                ("json", "1"),
            ],
        }
    }

    pub fn join<'a>(&self, other: &'a [QueryTuple]) -> QueryVec {
        let mut v = self.get();
        v.extend_from_slice(other);
        v
    }
}

#[async_trait]
pub trait Client: From<ClientBuilder<Self>> + ClientInformation {
    fn builder() -> ClientBuilder<Self> {
        ClientBuilder::new()
    }

    async fn get_by_id(&self, id: u32) -> Result<Option<Self::Post>, reqwest::Error>;
    async fn get(&self) -> Result<Vec<Self::Post>, reqwest::Error>;

    fn validate(_validates: ValidationType<'_, Self>) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl<T: Client + ClientInformation> ClientBuilder<T> {
    pub fn new() -> Self {
        Self {
            client: reqwest::lient::new(),
            tags: Tags(vec![]),
            limit: 100,
            url: T::URL.to_string(),
        }
    }

    pub fn any_tag(mut self, tag: Tag<T>) -> Self {
        self.tags.0.push(tag);
        self
    }

    pub fn tag<S: ToString>(self, tag: S) -> Self {
        self.any_tag(Tag::Plain(tag.to_string()))
    }

    pub fn sort(self, sort: Sort) -> Self {
        self.any_tag(Tag::Sort(sort))
    }

    pub fn random(self) -> Self {
        self.sort(Sort::Random)
    }

    pub fn rating(self, rating: T::Rating) -> Self {
        self.any_tag(Tag::Rating(rating))
    }

    pub fn blacklist_tag<S: ToString>(self, tag: S) -> Self {
        self.any_tag(Tag::Blacklist(tag.to_string()))
    }

    /// Set how many posts you want to retrieve (100 is the default and maximum)
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }

    /// Change the default url for the client
    pub fn default_url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    /// Convert the builder into the necessary client
    pub fn build(self) -> Result<T, ValidationError> {
        T::validate(ValidationType::Tags(&self.tags)).map(|_| T::from(self))
    }
}

impl<T: Client + ClientInformation> Default for ClientBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}
