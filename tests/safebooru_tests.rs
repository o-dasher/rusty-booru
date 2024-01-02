pub mod generic_tests;

#[cfg(test)]
mod safebooru {
    use booru_rs::{
        safebooru::{client::SafebooruClient, model::SafebooruRating},
        shared::{
            client::{DispatcherTrait, WithClientBuilder},
            model::Sort,
        },
    };

    #[tokio::test]
    async fn get_posts_with_tag() {
        let posts = SafebooruClient::builder()
            .query(|q| q.tag("kafuu_chino"))
            .unwrap()
            .get()
            .await;


        // SafebooruClient::builder().query(|q| q.tag()) // In this case the query should be
        // validated by the query function.
        // In the case of a generic client builder the syntax should be as follow:
        // GenericClient::builder().tag().validate(), so this pretty much means that the validation
        // should be mainly done on the QueryBuilder sttruct rather than on the dispatcher.

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_rating() {
        let posts = SafebooruClient::builder()
            .query(|q| q.tag("kafuu_chino").rating(SafebooruRating::General))
            .unwrap()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_sort() {
        let posts = SafebooruClient::builder()
            .query(|q| q.tag("kafuu_chino").sort(Sort::Rating))
            .unwrap()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_blacklist_tag() {
        let posts = SafebooruClient::builder()
            .query(|q| {
                q.tag("kafuu_chino")
                    .blacklist_tag(SafebooruRating::Explicit)
            })
            .unwrap()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_limit() {
        let posts = SafebooruClient::builder()
            .query(|q| q.tag("kafuu_chino").limit(3))
            .unwrap()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(posts.unwrap().len() == 3);
    }

    #[tokio::test]
    async fn get_posts_multiple_tags() {
        let posts = SafebooruClient::builder()
            .query(|q| q.tag("kafuu_chino").tag("bangs").limit(3))
            .unwrap()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_random_posts() {
        let posts = SafebooruClient::builder()
            .query(|q| q.tag("kafuu_chino").random())
            .unwrap()
            .get()
            .await;

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_post_by_id() {
        let post = SafebooruClient::builder()
            .dispatch()
            .get_by_id(4348760)
            .await;

        assert!(post.is_ok());
        assert_eq!(
            "3e407a7848804119f1064c2aac731545",
            post.unwrap().unwrap().hash
        );
    }

    #[test]
    fn parse_rating_tags() {
        assert_eq!("safe", SafebooruRating::Safe.to_string());
        assert_eq!("general", SafebooruRating::General.to_string());
        assert_eq!("questionable", SafebooruRating::Questionable.to_string());
        assert_eq!("explicit", SafebooruRating::Explicit.to_string());
    }

    #[test]
    fn parse_sort_tags() {
        assert_eq!("id", Sort::Id.to_string());
        assert_eq!("score", Sort::Score.to_string());
        assert_eq!("rating", Sort::Rating.to_string());
        assert_eq!("user", Sort::User.to_string());
        assert_eq!("height", Sort::Height.to_string());
        assert_eq!("width", Sort::Width.to_string());
        assert_eq!("source", Sort::Source.to_string());
        assert_eq!("updated", Sort::Updated.to_string());
    }
}
