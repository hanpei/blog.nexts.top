mod builder;
mod index;
mod markdown;
mod posts;
mod server;
mod templates;
pub use builder::*;
pub use server::*;

pub const BASE_PATH: &str = ".";
pub const STATIC_PATH: &str = "static";
pub const PAGE_DIR: &str = "pages";
pub const POSTS_DIR: &str = "posts";
pub const POST_TEMPLATE: &str = "post.html";
pub const INDEX_TEMPLATE: &str = "index.html";
pub const OUTPUT_PATH: &str = "dist";
