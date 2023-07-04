use async_trait::async_trait;
use derive_more::From;

use super::{Client, ClientBuilder};
use crate::model::gelbooru::*;

/// Client that sends requests to the Gelbooru API to retrieve the data.
#[derive(From)]
pub struct GelbooruClient<'a>(ClientBuilder<'a, GelbooruRating, Self>);

#[async_trait]
impl<'a> Client<'a, GelbooruRating> for GelbooruClient<'a> {
    type Post = GelbooruPost;

    const URL: &'static str = "https://gelbooru.com";
    const SORT: &'static str = "sort:";

    /// Directly get a post by its unique Id
    async fn get_by_id(&self, id: u32) -> Result<GelbooruPost, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url;

        let response = builder
            .client
            .get(format!("{url}/index.php"))
            .query(&[
                ("page", "dapi"),
                ("s", "post"),
                ("q", "index"),
                ("id", &id.to_string()),
                ("json", "1"),
            ])
            .send()
            .await?
            .json::<GelbooruResponse>()
            .await?;

        Ok(response.posts.into_iter().next().unwrap())
    }

    /// Pack the [`ClientBuilder`] and sent the request to the API to retrieve the posts
    async fn get(&self) -> Result<Vec<GelbooruPost>, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url;

        let response = builder
            .client
            .get(format!("{url}/index.php"))
            .query(&[
                ("page", "dapi"),
                ("s", "post"),
                ("q", "index"),
                ("limit", &builder.limit.to_string()),
                ("tags", &builder.tags.unpack()),
                ("json", "1"),
            ])
            .send()
            .await?
            .json::<GelbooruResponse>()
            .await?;

        Ok(response.posts)
    }
}
