use std::{fmt::Display, marker::PhantomData, sync::Arc};

use crate::generic::{BooruPost, Rating};

use super::{Sort, Tag, Tags, ValidationError};
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
    type Rating: From<Rating> + Display + Clone;
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

pub trait DispatcherTrait<T: ClientTypes> {
    /// Pack the [`ClientBuilder`] and sent the request to the API to retrieve the posts
    fn get_by_id(
        &self,
        id: u32,
    ) -> impl std::future::Future<Output = Result<Option<T::Post>, reqwest::Error>> + Send;

    /// Directly get a post by its unique Id
    fn get(&self)
        -> impl std::future::Future<Output = Result<Vec<T::Post>, reqwest::Error>> + Send;
}

pub trait WithCommonQuery {
    fn common_query_type() -> QueryLike;
}

pub trait ImplementedWithCommonQuery<T: ClientTypes + ClientInformation + QueryBuilderRules> {
    fn get_query(query: &ValidatedQuery<T>, query_mode: QueryMode) -> QueryVec;
}

impl<T: WithCommonQuery + ClientTypes + ClientInformation + QueryBuilderRules>
    ImplementedWithCommonQuery<T> for ClientQueryDispatcher<T>
{
    fn get_query(query: &ValidatedQuery<T>, query_mode: QueryMode) -> QueryVec {
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
                    ("limit", query.0.limit.to_string()),
                    ("tags", query.0.tags.unpack()),
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

#[derive(Clone)]
pub struct ClientQueryBuilder<T: ClientTypes + QueryBuilderRules> {
    pub tags: Tags<T>,
    pub limit: u32,
}

impl<T: ClientTypes + QueryBuilderRules + Clone> Default for ClientQueryBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ClientTypes + QueryBuilderRules + Clone> ClientQueryBuilder<T> {
    pub fn new() -> Self {
        Self {
            tags: Tags(Vec::new()),
            limit: 100,
        }
    }

    pub fn any_tag(&mut self, tag: Tag<T>) -> &mut Self {
        self.tags.0.push(tag);
        self
    }

    pub fn tag<S: ToString>(&mut self, tag: S) -> &mut Self {
        self.any_tag(Tag::Plain(tag.to_string()))
    }

    pub fn sort(&mut self, sort: Sort) -> &mut Self {
        self.any_tag(Tag::Sort(sort))
    }

    pub fn random(&mut self) -> &mut Self {
        self.sort(Sort::Random)
    }

    pub fn rating(&mut self, rating: T::Rating) -> &mut Self {
        self.any_tag(Tag::Rating(rating))
    }

    pub fn blacklist_tag<S: ToString>(&mut self, tag: S) -> &mut Self {
        self.any_tag(Tag::Blacklist(tag.to_string()))
    }

    /// Set how many posts you want to retrieve (100 is the default and maximum)
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = limit;
        self
    }

    pub fn validate(&mut self) -> Result<ValidatedQuery<T>, ValidationError> {
        T::validate(ValidationType::Tags(&self.tags))
            .map(|_| ValidatedQuery(Arc::new(self.clone())))
    }
}

#[derive(Clone)]
pub struct ValidatedQuery<T: QueryBuilderRules + ClientTypes>(pub Arc<ClientQueryBuilder<T>>);

pub trait QueryBuilderRules: ClientTypes + Sized {
    fn validate(_validates: ValidationType<'_, Self>) -> Result<(), ValidationError>;
}

impl<T: ClientTypes + QueryBuilderRules + Clone> ClientBuilder<T> {
    fn create_dispatcher(&self, query: ValidatedQuery<T>) -> ClientQueryDispatcher<T> {
        ClientQueryDispatcher {
            builder: self.to_owned(),
            query,
        }
    }

    pub fn query_raw(
        &self,
        query: &mut ClientQueryBuilder<T>,
    ) -> Result<ClientQueryDispatcher<T>, ValidationError> {
        query
            .validate()
            .map(|query| self.create_dispatcher(query.clone()))
    }

    pub fn query(
        &self,
        query_fn: impl Fn(&mut ClientQueryBuilder<T>) -> &mut ClientQueryBuilder<T>,
    ) -> Result<ClientQueryDispatcher<T>, ValidationError> {
        self.query_raw(query_fn(&mut ClientQueryBuilder::new()))
    }

    // Dispatches an empty query. Useful if you want to get a post by its id.
    pub fn dispatch(&self) -> ClientQueryDispatcher<T> {
        self.create_dispatcher(ValidatedQuery(Arc::new(ClientQueryBuilder::new())))
    }
}

pub struct ClientQueryDispatcher<T: ClientTypes + QueryBuilderRules> {
    pub builder: ClientBuilder<T>,
    pub query: ValidatedQuery<T>,
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
