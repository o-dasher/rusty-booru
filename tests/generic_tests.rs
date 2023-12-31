#[cfg(test)]
mod safebooru {
    use booru_rs::shared::builder::GenericClient;

    #[tokio::test]
    async fn get_posts_with_tag() {
        let posts = GenericClient::builder()
            .tag("kafuu_chino")
            .build()
            .unwrap()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }
}
