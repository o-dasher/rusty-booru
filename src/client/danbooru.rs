use super::{Client, ClientBuilder, ValidationType};
use crate::model::danbooru::*;
use anyhow::{ensure, Result};
use derive_more::From;

use async_trait::async_trait;
use reqwest::{header, header::HeaderMap};

// This is only here because of Danbooru, thanks Danbooru, really cool :)
pub fn get_headers() -> HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("PostmanRuntime/7.30.0"),
    );
    headers
}

/// Client that sends requests to the Danbooru API to retrieve the data.
#[derive(From)]
pub struct DanbooruClient(ClientBuilder<DanbooruRating, Self>);

#[async_trait]
impl Client<DanbooruRating> for DanbooruClient {
    type Post = DanbooruPost;

    const URL: &'static str = "https://danbooru.donmai.us";
    const SORT: &'static str = "order:";

    fn validate(
        builder: &ClientBuilder<DanbooruRating, Self>,
        validates: ValidationType,
    ) -> Result<()> {
        match validates {
            ValidationType::Tags => {
                ensure!(
                    builder.tags.len() <= 1,
                    "Danbooru only allows two tags per query"
                );
            }
        }

        Ok(())
    }

    /// Directly get a post by its unique Id
    async fn get_by_id(&self, id: u32) -> Result<Self::Post, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url.as_str();
        let response = builder
            .client
            .get(format!("{url}/posts/{id}.json"))
            .headers(get_headers())
            .send()
            .await?
            .json::<DanbooruPost>()
            .await?;
        Ok(response)
    }

    /// Pack the [`ClientBuilder`] and sent the request to the API to retrieve the posts
    async fn get(&self) -> Result<Vec<Self::Post>, reqwest::Error> {
        let builder = &self.0;
        let tag_string = builder.tags.join(" ");
        let url = builder.url.as_str();
        let response = builder
            .client
            .get(format!("{url}/posts.json"))
            .headers(get_headers())
            .query(&[
                ("limit", builder.limit.to_string().as_str()),
                ("tags", &tag_string),
            ])
            .send()
            .await?
            .json::<Vec<DanbooruPost>>()
            .await?;

        Ok(response)
    }
}
