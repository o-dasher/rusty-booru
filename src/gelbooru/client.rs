use derive_more::From;

use crate::shared::{
    client::{
        ClientBuilder, ClientInformation, ClientQueryDispatcher, ClientTypes, DispatcherTrait,
        ImplementedWithCommonQuery, QueryBuilderRules, QueryLike, QueryMode, ValidationType,
        WithCommonQuery,
    },
    ValidationError,
};

use super::*;

/// Client that sends requests to the Gelbooru API to retrieve the data.
#[derive(From, Clone)]
pub struct GelbooruClient(pub ClientBuilder<Self>);

impl ClientInformation for GelbooruClient {
    const URL: &'static str = "https://gelbooru.com";
    const SORT: &'static str = "sort:";
}

impl ClientTypes for GelbooruClient {
    type Post = GelbooruPost;
    type Rating = GelbooruRating;
}

impl WithCommonQuery for GelbooruClient {
    fn common_query_type() -> crate::shared::client::QueryLike {
        QueryLike::Gelbooru
    }
}

impl DispatcherTrait<GelbooruClient> for ClientQueryDispatcher<GelbooruClient> {
    async fn get_by_id(&self, id: u32) -> Result<Option<GelbooruPost>, reqwest::Error> {
        self.builder
            .client
            .get(format!("{}/index.php", &self.builder.url))
            .query(&Self::get_query(&self.query, QueryMode::Single(id)))
            .send()
            .await?
            .json::<GelbooruResponse>()
            .await
            .map(|r| r.posts.into_iter().next())
    }

    async fn get(&self) -> Result<Vec<GelbooruPost>, reqwest::Error> {
        self.builder
            .client
            .get(format!("{}/index.php", &self.builder.url))
            .query(&Self::get_query(&self.query, QueryMode::Multiple))
            .send()
            .await?
            .json::<GelbooruResponse>()
            .await
            .map(|r| r.posts)
    }
}

impl QueryBuilderRules for GelbooruClient {
    fn validate(_validates: ValidationType<'_, Self>) -> Result<(), ValidationError> {
        Ok(())
    }
}
