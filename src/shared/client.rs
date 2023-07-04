use std::fmt::Display;

use anyhow::Result;

use async_trait::async_trait;

use super::model::{Rating, Sort, Tag, Tags};

pub struct ClientBuilder<'a, R: Into<Rating> + Display, T: ClientInformation> {
    pub client: reqwest::Client,
    pub key: Option<String>,
    pub user: Option<String>,
    pub tags: Tags<'a, R, T>,
    pub limit: u32,
    pub url: &'a str,
}

pub enum ValidationType<'a, 'b, R: Into<Rating> + Display, T: ClientInformation> {
    Tags(&'b Tags<'a, R, T>),
}

pub trait ClientInformation {
    const URL: &'static str;
    const SORT: &'static str;
}

#[async_trait]
pub trait Client<'a, R: Into<Rating> + Display>:
    From<ClientBuilder<'a, R, Self>> + ClientInformation + 'a
{
    type Post;

    fn builder() -> ClientBuilder<'a, R, Self> {
        ClientBuilder::new()
    }

    async fn get_by_id(&self, id: u32) -> Result<Self::Post, reqwest::Error>;
    async fn get(&self) -> Result<Vec<Self::Post>, reqwest::Error>;

    fn validate(_validates: ValidationType<'a, '_, R, Self>) -> Result<()> {
        Ok(())
    }
}

impl<'a, R: Into<Rating> + Display, T: Client<'a, R> + ClientInformation> ClientBuilder<'a, R, T> {
    fn ensure_valid(&self, validates: ValidationType<'a, '_, R, T>) {
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
            url: T::URL,
        }
    }

    /// Set the API key and User for the requests (optional)
    pub fn set_credentials(mut self, key: String, user: String) -> Self {
        self.key = Some(key);
        self.user = Some(user);
        self
    }

    pub fn any_tag(mut self, tag: Tag<'a, R, T>) -> Self {
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

    pub fn rating(self, rating: R) -> Self {
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
    pub fn default_url(mut self, url: &'a str) -> Self {
        self.url = url;
        self
    }

    /// Convert the builder into the necessary client
    pub fn build(self) -> T {
        T::from(self)
    }
}

impl<'a, R: Into<Rating> + Display, T: Client<'a, R> + ClientInformation> Default
    for ClientBuilder<'a, R, T>
{
    fn default() -> Self {
        Self::new()
    }
}
