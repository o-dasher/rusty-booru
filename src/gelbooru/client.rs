use async_trait::async_trait;
use derive_more::From;

use crate::shared::client::{BaseQuery, Client, ClientBuilder, ClientInformation, QueryVec};

use super::model::*;

/// Client that sends requests to the Gelbooru API to retrieve the data.
#[derive(From)]
pub struct GelbooruClient(ClientBuilder<Self>);

impl ClientInformation for GelbooruClient {
    const URL: &'static str = "https://gelbooru.com";
    const SORT: &'static str = "sort:";

    type Post = GelbooruPost;
    type Rating = GelbooruRating;
}

#[async_trait]
impl Client for GelbooruClient {
    /// Directly get a post by its unique Id
    async fn get_by_id(&self, id: u32) -> Result<Option<GelbooruPost>, reqwest::Error> {
        let builder = &self.0;

        builder
            .client
            .get(format!("{}/index.php", &builder.url))
            .query(&BaseQuery::GelbooruLike.join(&[("id", &id.to_string())]))
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
            .query(&BaseQuery::GelbooruLike.join(&[
                ("limit", &builder.limit.to_string()),
                ("tags", &builder.tags.unpack()),
            ]))
            .send()
            .await?
            .json::<GelbooruResponse>()
            .await
            .map(|r| r.posts)
    }
}
