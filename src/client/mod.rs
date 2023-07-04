use std::{convert::Infallible, fmt::Display, marker::PhantomData};

use anyhow::Result;

use async_trait::async_trait;
use derive_is_enum_variant::is_enum_variant;

use self::generic::{Rating, Sort};

pub mod danbooru;
pub mod gelbooru;
pub mod generic;
pub mod safebooru;

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

pub struct ClientBuilder<'a, R: Into<Rating> + Display, T: Client<'a, R>> {
    client: reqwest::Client,
    key: Option<String>,
    user: Option<String>,
    tags: Vec<Tag<'a, R, T>>,
    limit: u32,
    url: &'a str,
    _marker_t: std::marker::PhantomData<T>,
    _marker_r: std::marker::PhantomData<R>,
}

pub enum ValidationType<'a, 'b, R: Into<Rating> + Display, T: Client<'a, R>> {
    Tags(&'b Vec<Tag<'a, R, T>>),
}

#[async_trait]
pub trait Client<'a, R: Into<Rating> + Display>: From<ClientBuilder<'a, R, Self>>
where
    Self: 'a,
{
    type Post;

    const URL: &'static str;
    const SORT: &'static str;

    fn builder() -> ClientBuilder<'a, R, Self> {
        ClientBuilder::new()
    }

    async fn get_by_id(&self, id: u32) -> Result<Self::Post, reqwest::Error>;
    async fn get(&self) -> Result<Vec<Self::Post>, reqwest::Error>;

    fn validate(_validates: ValidationType<'a, '_, R, Self>) -> Result<()> {
        Ok(())
    }
}

impl<'a, R: Into<Rating> + Display, T: Client<'a, R>> ClientBuilder<'a, R, T> {
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

    pub fn any_tag(mut self, tag: Tag<'a, R, T>) -> Self {
        // Danbooru has an special case for plain tags.
        // it must have at max 2 plain tags.
        if let Tag::Plain(..) = tag {
            self.ensure_valid(ValidationType::Tags(&self.tags))
        }

        self.tags.push(tag);

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

impl<'a, R: Into<Rating> + Display, T: Client<'a, R>> Default for ClientBuilder<'a, R, T> {
    fn default() -> Self {
        Self::new()
    }
}
