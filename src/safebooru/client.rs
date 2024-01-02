use derive_more::From;

use crate::shared::client::{
    ClientBuilder, ClientInformation, ClientQueryDispatcher, ClientTypes, DispatcherTrait,
    ImplementedWithCommonQuery, QueryBuilderRules, QueryLike, QueryMode, WithCommonQuery,
};

use super::*;

#[derive(From)]
pub struct SafebooruClient(pub ClientBuilder<Self>);

impl ClientInformation for SafebooruClient {
    const URL: &'static str = "https://safebooru.org";
    const SORT: &'static str = "sort:";
}

impl ClientTypes for SafebooruClient {
    type Post = SafebooruPost;
    type Rating = SafebooruRating;
}

impl WithCommonQuery for SafebooruClient {
    fn common_query_type() -> QueryLike {
        QueryLike::Gelbooru
    }
}

impl DispatcherTrait<SafebooruClient> for ClientQueryDispatcher<SafebooruClient> {
    async fn get_by_id(&self, id: u32) -> Result<Option<SafebooruPost>, reqwest::Error> {
        self.builder
            .client
            .get(&format!("{}/index.php", &self.builder.url))
            .query(&Self::get_query(&self.query, QueryMode::Single(id)))
            .send()
            .await?
            .json::<Vec<SafebooruPost>>()
            .await
            .map(|r| r.into_iter().next())
    }

    async fn get(&self) -> Result<Vec<SafebooruPost>, reqwest::Error> {
        self.builder
            .client
            .get(format!("{}/index.php", &self.builder.url))
            .query(&Self::get_query(&self.query, QueryMode::Multiple))
            .send()
            .await?
            .json::<Vec<SafebooruPost>>()
            .await
    }
}

impl QueryBuilderRules for SafebooruClient {}
