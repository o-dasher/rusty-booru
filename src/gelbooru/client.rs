use async_trait::async_trait;
use derive_more::From;

use crate::shared::client::{
    Client, ClientBuilder, ClientInformation, ClientTypes, QueryLike, QueryMode, WithCommonQuery,
};

use super::model::*;

/// Client that sends requests to the Gelbooru API to retrieve the data.
#[derive(From)]
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

#[async_trait]
impl Client for GelbooruClient {
    /// Directly get a post by its unique Id
    async fn get_by_id(&self, id: u32) -> Result<Option<GelbooruPost>, reqwest::Error> {
        let builder = &self.0;

        builder
            .client
            .get(format!("{}/index.php", &builder.url))
            .query(&self.get_query(builder, QueryMode::Single(id)))
            .send()
            .await?
            .json::<GelbooruResponse>()
            .await
            .map(|r| r.posts.into_iter().next())
    }

    /// Pack the [`ClientBuilder`] and sent the request to the API to retrieve the posts
    async fn get(&self) -> Result<Vec<GelbooruPost>, reqwest::Error> {
        let builder = &self.0;

        builder
            .client
            .get(format!("{}/index.php", &builder.url))
            .query(&self.get_query(builder, QueryMode::Multiple))
            .send()
            .await?
            .json::<GelbooruResponse>()
            .await
            .map(|r| r.posts)
    }
}
