use async_trait::async_trait;
use derive_more::From;

use super::{Client, ClientBuilder};
use crate::{model::safebooru::SafebooruPost, safebooru::SafebooruRating};

#[derive(From)]
pub struct SafebooruClient<'a>(ClientBuilder<'a, SafebooruRating, Self>);

#[async_trait]
impl<'a> Client<'a, SafebooruRating> for SafebooruClient<'a> {
    type Post = SafebooruPost;

    const URL: &'static str = "https://safebooru.org";
    const SORT: &'static str = "sort:";

    async fn get_by_id(&self, id: u32) -> Result<Self::Post, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url;
        let response = builder
            .client
            .get(format!("{url}/index.php"))
            .query(&[
                ("page", "dapi"),
                ("s", "post"),
                ("q", "index"),
                ("id", id.to_string().as_str()),
                ("json", "1"),
            ])
            .send()
            .await?
            .json::<Vec<SafebooruPost>>()
            .await?;

        // FIXME: Assumes there is a post with the given id. Same is true for the
        // Gelbooru client.
        Ok(response.into_iter().next().unwrap())
    }

    async fn get(&self) -> Result<Vec<Self::Post>, reqwest::Error> {
        let builder = &self.0;
        let url = builder.url;
        let tags = builder.tags.join(" ");
        Ok(builder
            .client
            .get(format!("{url}/index.php"))
            .query(&[
                ("page", "dapi"),
                ("s", "post"),
                ("q", "index"),
                ("limit", builder.limit.to_string().as_str()),
                ("tags", &tags),
                ("json", "1"),
            ])
            .send()
            .await?
            .json::<Vec<SafebooruPost>>()
            .await?)
    }
}
