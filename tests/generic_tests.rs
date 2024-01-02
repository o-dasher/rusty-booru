#[cfg(test)]
mod generic {
    use booru_rs::generic::client::{BooruOption, GenericClient};

    #[tokio::test]
    async fn get_posts_with_tag() {
        let posts = GenericClient::query()
            .tag("kafuu_chino")
            .validate()
            .unwrap()
            .get(BooruOption::Gelbooru)
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }
}
