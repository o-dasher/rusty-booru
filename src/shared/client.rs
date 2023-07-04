use std::fmt::Display;

use anyhow::Result;

use async_trait::async_trait;

use super::model::{Rating, Sort, Tag, Tags};

pub struct ClientBuilder<T: ClientInformation> {
    pub client: reqwest::Client,
    pub key: Option<String>,
    pub user: Option<String>,
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

    type Rating: Into<Rating> + Display;
    type Post;
}

#[async_trait]
pub trait Client: From<ClientBuilder<Self>> + ClientInformation {
    fn builder() -> ClientBuilder<Self> {
        ClientBuilder::new()
    }

    async fn get_by_id(&self, id: u32) -> Result<Self::Post, reqwest::Error>;
    async fn get(&self) -> Result<Vec<Self::Post>, reqwest::Error>;

    fn validate(_validates: ValidationType<'_, Self>) -> Result<()> {
        Ok(())
    }
}

impl<T: Client + ClientInformation> ClientBuilder<T> {
    fn ensure_valid(&self, validates: ValidationType<'_, T>) {
        if let Err(e) = T::validate(validates) {
            panic!("{}", e)
        }
    }

    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            key: None,
            user: None,
            tags: Tags(vec![]),
            limit: 100,
            url: T::URL.to_string(),
        }
    }

    /// Set the API key and User for the requests (optional)
    pub fn set_credentials(mut self, key: String, user: String) -> Self {
        self.key = Some(key);
        self.user = Some(user);
        self
    }

    pub fn any_tag(mut self, tag: Tag<T>) -> Self {
        // Danbooru has an special case for plain tags.
        // it must have at max 2 plain tags.
        if let Tag::Plain(..) = tag {
            self.ensure_valid(ValidationType::Tags(&self.tags))
        }

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
    pub fn build(self) -> T {
        T::from(self)
    }
}

impl<T: Client + ClientInformation> Default for ClientBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}
