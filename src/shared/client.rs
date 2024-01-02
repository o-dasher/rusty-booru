use std::{fmt::Display, marker::PhantomData};

use super::model::{BooruPost, Rating, Sort, Tag, Tags, ValidationError};
use async_trait::async_trait;
use itertools::Itertools;

pub struct ClientBuilder<T: ClientTypes> {
    pub client: reqwest::Client,
    pub url: String,

    _marker: PhantomData<T>,
}

impl<T: ClientTypes> Clone for ClientBuilder<T> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            url: self.url.clone(),
            _marker: self._marker,
        }
    }
}

pub enum ValidationType<'a, T: ClientTypes> {
    Tags(&'a Tags<T>),
}

pub trait ClientInformation {
    const URL: &'static str;
    const SORT: &'static str;
}

pub trait ClientTypes {
    type Rating: From<Rating> + Display;
    type Post: Into<BooruPost>;
}

pub type QueryVec = Vec<(String, String)>;

pub enum QueryLike {
    Gelbooru,
}

pub enum QueryMode {
    Single(u32),
    Multiple,
}

pub trait WithClientBuilder<T: ClientTypes> {
    fn builder() -> ClientBuilder<T>;
}

impl<T: ClientInformation + ClientTypes + From<ClientBuilder<T>>> WithClientBuilder<T> for T {
    fn builder() -> ClientBuilder<T> {
        ClientBuilder::new()
    }
}

#[async_trait]
pub trait DispatcherTrait<T: ClientTypes> {
    /// Pack the [`ClientBuilder`] and sent the request to the API to retrieve the posts
    async fn get_by_id(&self, id: u32) -> Result<Option<T::Post>, reqwest::Error>;

    /// Directly get a post by its unique Id
    async fn get(&self) -> Result<Vec<T::Post>, reqwest::Error>;
}

pub trait WithCommonQuery {
    fn common_query_type() -> QueryLike;
}

pub trait ImplementedWithCommonQuery<T: ClientTypes + ClientInformation> {
    fn get_query(query: &ClientQueryBuilder<T>, query_mode: QueryMode) -> QueryVec;
}

impl<T: WithCommonQuery + ClientTypes + ClientInformation> ImplementedWithCommonQuery<T>
    for ClientQueryDispatcher<T>
{
    fn get_query(query: &ClientQueryBuilder<T>, query_mode: QueryMode) -> QueryVec {
        let query_type = T::common_query_type();

        let mut base = match query_type {
            QueryLike::Gelbooru => vec![
                ("page", "dapi"),
                ("s", "post"),
                ("q", "index"),
                ("json", "1"),
            ],
        }
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect_vec();

        let extension = match query_type {
            QueryLike::Gelbooru => match query_mode {
                QueryMode::Single(id) => vec![("id", id.to_string())],
                QueryMode::Multiple => vec![
                    ("limit", query.limit.to_string()),
                    ("tags", query.tags.unpack()),
                ],
            },
        }
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect_vec();

        base.extend(extension);
        base
    }
}

// ClientQueryBuilder is structured separately for a reason, it can't hold references to a client
// builder that has an url. This because the generic client does not have a proper url, and thus
// can't have the ClientInformation trait. By structuring it separately we can create a "contained"
// query, that can then be passed on to the proper Client that will be able to figure out the
// proper way to handle the used query at runtime.

pub struct ClientQueryBuilder<T: ClientTypes> {
    pub tags: Tags<T>,
    pub limit: u32,
}

impl<T: ClientTypes> Default for ClientQueryBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ClientTypes> ClientQueryBuilder<T> {
    pub fn new() -> Self {
        Self {
            tags: Tags(Vec::new()),
            limit: 100,
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
}

pub trait QueryBuilderRules: ClientTypes + Sized {
    fn validate(_validates: ValidationType<'_, Self>) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl<T: ClientTypes + QueryBuilderRules> ClientBuilder<T> {
    fn create_dispatcher(&self, query: ClientQueryBuilder<T>) -> ClientQueryDispatcher<T> {
        ClientQueryDispatcher {
            builder: self.to_owned(),
            query,
        }
    }

    pub fn query(
        &self,
        query_fn: impl Fn(ClientQueryBuilder<T>) -> ClientQueryBuilder<T>,
    ) -> Result<ClientQueryDispatcher<T>, ValidationError> {
        self.query_raw(query_fn(ClientQueryBuilder::new()))
    }

    pub fn query_raw(
        &self,
        query: ClientQueryBuilder<T>,
    ) -> Result<ClientQueryDispatcher<T>, ValidationError> {
        T::validate(ValidationType::Tags(&query.tags)).map(|()| self.create_dispatcher(query))
    }

    pub fn dispatch(&self) -> ClientQueryDispatcher<T> {
        self.create_dispatcher(ClientQueryBuilder::new())
    }
}

pub struct ClientQueryDispatcher<T: ClientTypes> {
    pub builder: ClientBuilder<T>,
    pub query: ClientQueryBuilder<T>,
}

impl<T: ClientInformation + ClientTypes> ClientBuilder<T> {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            url: T::URL.to_string(),

            _marker: PhantomData,
        }
    }

    /// Change the default url for the client
    pub fn default_url(&mut self, url: &str) -> &mut Self {
        self.url = url.to_string();
        self
    }
}

impl<T: ClientTypes + ClientInformation> Default for ClientBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}
