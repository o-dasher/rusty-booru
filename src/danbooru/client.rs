use derive_more::From;
use reqwest::{header, header::HeaderMap, Response};

use super::*;
use crate::shared::{self, client::*};

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
#[derive(From, Clone)]
pub struct DanbooruClient(pub ClientBuilder<Self>);

impl ClientInformation for DanbooruClient {
    const URL: &'static str = "https://danbooru.donmai.us";
    const SORT: &'static str = "order:";
}

impl ClientTypes for DanbooruClient {
    type Rating = DanbooruRating;
    type Post = DanbooruPost;
}

#[derive(Deserialize, Debug, thiserror::Error, Display)]
pub enum DanbooruError {
    #[serde(rename = "PostQuery::TagLimitError")]
    TagLimitError,
}

#[derive(Deserialize)]
struct DanbooruErrorStruct {
    pub error: DanbooruError,
}

impl From<DanbooruErrorStruct> for shared::Error {
    fn from(value: DanbooruErrorStruct) -> Self {
        value.error.into()
    }
}

async fn send_error<T>(response: Response) -> Result<T, shared::Error> {
    Err(response
        .json::<DanbooruErrorStruct>()
        .await
        .map(Into::into)?)
}

impl DispatcherTrait<DanbooruClient> for ClientQueryDispatcher<DanbooruClient> {
    async fn get_by_id(&self, id: u32) -> Result<Option<DanbooruPost>, shared::Error> {
        let response = self
            .builder
            .client
            .get(format!("{}/posts/{id}.json", self.builder.url))
            .headers(get_headers())
            .send()
            .await?;

        if response.status().is_success() {
            response
                .json::<DanbooruPost>()
                .await
                .map(Into::into)
                .map_err(Into::into)
        } else {
            send_error(response).await?
        }
    }

    async fn get(&self) -> Result<Vec<DanbooruPost>, shared::Error> {
        let response = self
            .builder
            .client
            .get(format!("{}/posts.json", self.builder.url))
            .headers(get_headers())
            .query(&[
                ("limit", &self.query.limit.to_string()),
                ("tags", &self.query.tags.unpack()),
            ])
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<Vec<DanbooruPost>>().await?)
        } else {
            send_error(response).await?
        }
    }
}
