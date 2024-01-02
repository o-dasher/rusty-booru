use crate::{
    danbooru::client::DanbooruClient,
    gelbooru::client::GelbooruClient,
    safebooru::client::SafebooruClient,
    shared::client::{DispatcherTrait, QueryBuilderRules, WithClientBuilder},
};

use super::{
    client::{
        ClientBuilder, ClientInformation, ClientQueryBuilder, ClientQueryDispatcher, ClientTypes,
        ValidatedQuery, ValidationType,
    },
    model::{BooruPost, Rating, Tag, ValidationError},
};
use derive_more::From;

pub struct GenericClient(ClientBuilder<Self>);

impl ClientTypes for GenericClient {
    type Rating = Rating;
    type Post = BooruPost;
}

pub enum BooruOption {
    Gelbooru,
    Safebooru,
    Danbooru,
}

#[derive(thiserror::Error, Debug, From)]
pub enum GenericClientError {
    #[error(transparent)]
    Reqwest(reqwest::Error),

    #[error(transparent)]
    ValidationErrror(ValidationError),
}

impl<T: ClientTypes> From<&Tag<GenericClient>> for Tag<T> {
    fn from(val: &Tag<GenericClient>) -> Self {
        match val {
            Tag::Plain(s) => Tag::Plain(s.clone()),
            Tag::Blacklist(s) => Tag::Blacklist(s.clone()),
            Tag::Rating(s) => Tag::Rating(T::Rating::from(s.clone())),
            Tag::Sort(s) => Tag::Sort(s.clone()),
        }
    }
}

macro_rules! handle_request {
    (@ $t:ident, $($args:expr,)*) => {
        request::<$t>($($args,)*).await
    };

    ($booru_option:expr, ($($args:expr),*)) => {
        match $booru_option {
            BooruOption::Gelbooru => handle_request!(@ GelbooruClient, $($args,)*),
            BooruOption::Safebooru => handle_request!(@ SafebooruClient, $($args,)*),
            BooruOption::Danbooru => handle_request!(@ DanbooruClient, $($args,)*),
        }
    }
}

impl ValidatedQuery<GenericClient> {
    fn convert<T: ClientTypes + ClientInformation + QueryBuilderRules>(
        &self,
    ) -> ClientQueryBuilder<T> {
        let mut query = ClientQueryBuilder::new();

        for tag in self.0.tags.0.iter() {
            query = query.tag::<Tag<T>>(tag.into());
        }

        query
    }

    pub async fn get_by_id(
        &self,
        id: u32,
        booru: BooruOption,
    ) -> Result<Option<BooruPost>, GenericClientError> {
        async fn request<
            T: ClientTypes + ClientInformation + QueryBuilderRules + WithClientBuilder<T>,
        >(
            query: &ValidatedQuery<GenericClient>,
            id: u32,
        ) -> Result<Option<BooruPost>, GenericClientError>
        where
            ClientQueryDispatcher<T>: DispatcherTrait<T>,
        {
            T::builder()
                .query_raw(query.convert())?
                .get_by_id(id)
                .await
                .map(|v| v.map(Into::into))
                .map_err(Into::into)
        }

        handle_request!(booru, (self, id))
    }

    pub async fn get(&self, booru: BooruOption) -> Result<Vec<BooruPost>, GenericClientError> {
        async fn request<
            T: ClientTypes + ClientInformation + QueryBuilderRules + WithClientBuilder<T>,
        >(
            query: &ValidatedQuery<GenericClient>,
        ) -> Result<Vec<BooruPost>, GenericClientError>
        where
            ClientQueryDispatcher<T>: DispatcherTrait<T>,
        {
            T::builder()
                .query_raw(query.convert())?
                .get()
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(Into::into).collect())
        }

        handle_request!(booru, (self))
    }
}

impl QueryBuilderRules for GenericClient {
    fn validate(_validates: ValidationType<'_, Self>) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl GenericClient {
    pub fn query() -> ClientQueryBuilder<GenericClient> {
        ClientQueryBuilder::new()
    }
}

