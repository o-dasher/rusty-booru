use derive_more::From;

use crate::{
    generic::AutoCompleteItem,
    shared::{
        self,
        client::{
            ClientBuilder, ClientInformation, ClientQueryDispatcher, ClientTypes,
            ImplementedWithCommonQuery, QueryDispatcher, QueryLike, QueryMode, WithCommonQuery,
        },
    },
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

impl QueryDispatcher<GelbooruClient> for ClientQueryDispatcher<GelbooruClient> {
    async fn get_autocomplete<In: Into<String> + Send>(
        &self,
        input: In,
    ) -> Result<Vec<crate::generic::AutoCompleteItem>, reqwest::Error> {
        self.builder
            .client
            .get(format!("{}/index.php", self.builder.url))
            .query(&[
                ("limit", self.query.limit.to_string().as_str()),
                ("page", "autocomplete2"),
                ("type", "tag_query"),
                ("term", &input.into()),
            ])
            .send()
            .await?
            .json::<Vec<AutoCompleteItem>>()
            .await
    }

    async fn get_by_id(&self, id: u32) -> Result<Option<GelbooruPost>, shared::Error> {
        self.builder
            .client
            .get(format!("{}/index.php", &self.builder.url))
            .query(&Self::get_query(QueryMode::Single(id)))
            .send()
            .await?
            .json::<GelbooruResponse>()
            .await
            .map(|r| r.posts.into_iter().next())
            .map_err(Into::into)
    }

    async fn get(&self) -> Result<Vec<GelbooruPost>, shared::Error> {
        self.builder
            .client
            .get(format!("{}/index.php", &self.builder.url))
            .query(&Self::get_query(QueryMode::Multiple(&self.query)))
            .send()
            .await?
            .json::<GelbooruResponse>()
            .await
            .map(|r| r.posts)
            .map_err(Into::into)
    }
}
