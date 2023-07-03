use std::fmt::Display;

use anyhow::Result;

use async_trait::async_trait;

use self::generic::{Rating, Sort};

pub mod danbooru;
pub mod gelbooru;
pub mod generic;
pub mod safebooru;

pub struct ClientBuilder<'a, R: Into<Rating> + Display, T: Client<'a, R>> {
    client: reqwest::Client,
    key: Option<String>,
    user: Option<String>,
    tags: Vec<String>,
    limit: u32,
    url: &'a str,
    _marker_t: std::marker::PhantomData<T>,
    _marker_r: std::marker::PhantomData<R>,
}

pub enum ValidationType<'a> {
    Tags(&'a Vec<String>),
}

#[async_trait]
pub trait Client<'a, R: Into<Rating> + Display>: From<ClientBuilder<'a, R, Self>> {
    type Post;

    const URL: &'static str;
    const SORT: &'static str;

    fn builder() -> ClientBuilder<'a, R, Self> {
        ClientBuilder::new()
    }

    async fn get_by_id(&self, id: u32) -> Result<Self::Post, reqwest::Error>;
    async fn get(&self) -> Result<Vec<Self::Post>, reqwest::Error>;

    fn validate(_validates: ValidationType) -> Result<()> {
        Ok(())
    }
}

impl<'a, R: Into<Rating> + Display, T: Client<'a, R>> ClientBuilder<'a, R, T> {
    fn ensure_valid(&self, validates: ValidationType) {
        if let Err(e) = T::validate(validates) {
            panic!("{}", e)
        }
    }

    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            key: None,
            user: None,
            tags: vec![],
            limit: 100,
            url: T::URL,
            _marker_r: std::marker::PhantomData,
            _marker_t: std::marker::PhantomData,
        }
    }

    /// Set the API key and User for the requests (optional)
    pub fn set_credentials(mut self, key: String, user: String) -> Self {
        self.key = Some(key);
        self.user = Some(user);
        self
    }

    fn unchecked_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    /// Add a tag to the query
    pub fn tag<S: Into<String>>(self, tag: S) -> Self {
        self.ensure_valid(ValidationType::Tags(&self.tags));
        self.unchecked_tag(tag.into())
    }

    /// Add the client compatible rating.
    pub fn rating(self, rating: R) -> Self {
        self.unchecked_tag(format!("rating:{}", rating))
    }

    /// Set how many posts you want to retrieve (100 is the default and maximum)
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }

    /// Retrieves the posts in a random order
    pub fn random(self) -> Self {
        self.unchecked_tag(format!("{}:random", T::SORT))
    }

    /// Add a [`Sort`] to the query
    pub fn sort(self, order: Sort) -> Self {
        self.unchecked_tag(format!("{}:{}", T::SORT, order))
    }

    /// Blacklist a tag from the query
    pub fn blacklist_tag<S: Display>(self, tag: S) -> Self {
        self.unchecked_tag(format!("-{}", tag))
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

impl<'a, R: Into<Rating> + Display, T: Client<'a, R>> Default for ClientBuilder<'a, R, T> {
    fn default() -> Self {
        Self::new()
    }
}
