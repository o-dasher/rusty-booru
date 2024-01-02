//! ### Usage
//! ```
//! use booru_rs::{danbooru::{client::DanbooruClient, DanbooruRating}};
//! use booru_rs::shared::{client::{WithClientBuilder, DispatcherTrait}, Sort};
//!
//! #[tokio::main]
//! async fn main() {
//!     let posts = DanbooruClient::builder()
//!         .default_url("https://testbooru.donmai.us")
//!         .query(|q| {
//!             q.rating(DanbooruRating::General)
//!                 .sort(Sort::Score)
//!                 .limit(10)
//!         })
//!         .unwrap()
//!         .get()
//!         .await
//!         .expect("There was an error. (•-•)");
//!
//!     match posts.first() {
//!         Some(post) => println!("{:?}", post),
//!         None => panic!("Well... \"No posts found?\""),
//!     }
//! }
//! ```

pub mod danbooru;
pub mod gelbooru;
pub mod safebooru;
pub mod shared;
