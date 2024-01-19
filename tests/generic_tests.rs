#[cfg(test)]
mod generic {
    use rusty_booru::generic::client::{BooruOption, GenericClient};
    use strum::IntoEnumIterator;

    #[tokio::test]
    async fn get_posts_with_tag() {
        let posts = GenericClient::query()
            .tag("kafuu_chino")
            .get(BooruOption::Gelbooru)
            .await;

        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn assert_file_url_safebooru() {
        let post = GenericClient::query()
            .get_by_id(4683505, BooruOption::Safebooru)
            .await
            .unwrap()
            .unwrap();

        dbg!(&post.file_url);
        assert!(
            post.file_url.unwrap()
                == "https://safebooru.org/images/4491/d0e26173ad1896ca7c187c85a9d38f55329927b9.jpg"
        )
    }

    #[tokio::test]
    async fn get_autocomplete() {
        for booru in BooruOption::iter() {
            let tags = GenericClient::query().get_autocomplete(booru, "f").await;

            assert!(tags.is_ok());
            assert!(!tags.unwrap().is_empty());
        }
    }
}
