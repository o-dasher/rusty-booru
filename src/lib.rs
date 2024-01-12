//! ### Usage
//! ```
//! use rusty_booru::{danbooru::{client::DanbooruClient, DanbooruRating}};
//! use rusty_booru::shared::{client::{WithClientBuilder, DispatcherTrait}, Sort};
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
pub mod generic;
