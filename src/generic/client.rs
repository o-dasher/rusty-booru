use strum::EnumIter;

use crate::{
    danbooru::client::DanbooruClient,
    gelbooru::client::GelbooruClient,
    generic::AutoCompleteItem,
    safebooru::client::SafebooruClient,
    shared::{
        self,
        client::{
            ClientInformation, ClientQueryBuilder, ClientQueryDispatcher, ClientTypes,
            QueryDispatcher, WithClientBuilder,
        },
        Tag,
    },
};

use super::{BooruPost, Rating};

#[derive(Debug, Clone)]
pub struct GenericClient();

impl ClientTypes for GenericClient {
    type Rating = Rating;
    type Post = BooruPost;
}

#[derive(EnumIter)]
pub enum BooruOption {
    Gelbooru,
    Safebooru,
    Danbooru,
}

impl<T: ClientTypes> From<&Tag<GenericClient>> for Tag<T> {
    fn from(val: &Tag<GenericClient>) -> Self {
        match val {
            Tag::Plain(s) => Tag::Plain(s.clone()),
            Tag::Blacklist(s) => Tag::Blacklist(s.clone()),
            Tag::Rating(s) => Tag::Rating(T::Rating::from(s.clone())),
            Tag::Sort(s) => Tag::Sort(s.clone()),
        }
    }
}

macro_rules! handle_request {
    (@ $t:ident, ($($args:expr),*), ($($gen:ty),*)) => {
        request::<$t, $($gen,)*>($($args,)*).await
    };

    ($booru_option:expr, ($($args:expr),*), ($($gen:ty),*)) => {
        match $booru_option {
            BooruOption::Gelbooru => handle_request!(@ GelbooruClient, ($($args),*), ($($gen),*)),
            BooruOption::Safebooru => handle_request!(@ SafebooruClient, ($($args),*), ($($gen),*)),
            BooruOption::Danbooru => handle_request!(@ DanbooruClient, ($($args),*), ($($gen),*)),
        }
    };

    ($booru_option:expr, ($($args:expr),*)) => {
        handle_request!($booru_option, ($($args),*), ())
    }
}

impl ClientQueryBuilder<GenericClient> {
    fn convert<T: ClientTypes + ClientInformation + Clone>(&self) -> ClientQueryBuilder<T> {
        let mut query = ClientQueryBuilder::new();

        for tag in self.tags.0.iter() {
            query.tag::<Tag<T>>(tag.into());
        }

        query
    }

    pub async fn get_autocomplete<In: Into<String> + Send>(
        &self,
        booru: BooruOption,
        input: In,
    ) -> Result<Vec<AutoCompleteItem>, reqwest::Error> {
        async fn request<
            T: ClientTypes + ClientInformation + WithClientBuilder<T> + Clone,
            In: Into<String> + Send,
        >(
            query: &ClientQueryBuilder<GenericClient>,
            input: In,
        ) -> Result<Vec<AutoCompleteItem>, reqwest::Error>
        where
            ClientQueryDispatcher<T>: QueryDispatcher<T>,
        {
            T::builder()
                .query_raw(&mut query.convert())
                .get_autocomplete(input)
                .await
                .map_err(Into::into)
        }

        handle_request!(booru, (self, input), (In))
    }

    pub async fn get_by_id(
        &self,
        id: u32,
        booru: BooruOption,
    ) -> Result<Option<BooruPost>, shared::Error> {
        async fn request<T: ClientTypes + ClientInformation + WithClientBuilder<T> + Clone>(
            query: &ClientQueryBuilder<GenericClient>,
            id: u32,
        ) -> Result<Option<BooruPost>, shared::Error>
        where
            ClientQueryDispatcher<T>: QueryDispatcher<T>,
        {
            T::builder()
                .query_raw(&mut query.convert())
                .get_by_id(id)
                .await
                .map(|v| v.map(Into::into))
                .map_err(Into::into)
        }

        handle_request!(booru, (self, id))
    }

    pub async fn get(&self, booru: BooruOption) -> Result<Vec<BooruPost>, shared::Error> {
        async fn request<T: ClientTypes + ClientInformation + WithClientBuilder<T> + Clone>(
            query: &ClientQueryBuilder<GenericClient>,
        ) -> Result<Vec<BooruPost>, shared::Error>
        where
            ClientQueryDispatcher<T>: QueryDispatcher<T>,
        {
            T::builder()
                .query_raw(&mut query.convert())
                .get()
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(Into::into).collect())
        }

        handle_request!(booru, (self))
    }
}

impl GenericClient {
    pub fn query() -> ClientQueryBuilder<GenericClient> {
        ClientQueryBuilder::new()
    }
}
