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

#[derive(From, Debug, Clone)]
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

impl QueryDispatcher<SafebooruClient> for ClientQueryDispatcher<SafebooruClient> {
    async fn get_autocomplete<In: Into<String> + Send>(
        &self,
        input: In,
    ) -> Result<Vec<crate::generic::AutoCompleteItem>, reqwest::Error> {
        self.builder
            .client
            .get(format!("{}/autocomplete.php", self.builder.url))
            .query(&[("q", input.into())])
            .send()
            .await?
            .json::<Vec<AutoCompleteItem>>()
            .await
    }

    async fn get_by_id(&self, id: u32) -> Result<Option<SafebooruPost>, shared::Error> {
        self.builder
            .client
            .get(&format!("{}/index.php", &self.builder.url))
            .query(&Self::get_query(QueryMode::Single(id)))
            .send()
            .await?
            .json::<Vec<SafebooruPost>>()
            .await
            .map(|r| r.into_iter().next())
            .map_err(Into::into)
    }

    async fn get(&self) -> Result<Vec<SafebooruPost>, shared::Error> {
        self.builder
            .client
            .get(format!("{}/index.php", &self.builder.url))
            .query(&Self::get_query(QueryMode::Multiple(&self.query)))
            .send()
            .await?
            .json::<Vec<SafebooruPost>>()
            .await
            .map_err(Into::into)
    }
}
