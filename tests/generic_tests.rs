#[cfg(test)]
mod generic {
    use rusty_booru::generic::client::{BooruOption, GenericClient};

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

    #[tokio::test]
    async fn assert_file_url_safebooru() {
        let post = GenericClient::query()
            .validate()
            .unwrap()
            .get_by_id(4683505, BooruOption::Safebooru)
            .await
            .unwrap()
            .unwrap();

        dbg!(&post.file_url);
        assert!(
            post.file_url
                == "https://safebooru.org/images/d0e26173ad1896ca7c187c85a9d38f55329927b9.jpg"
        )
    }
}
