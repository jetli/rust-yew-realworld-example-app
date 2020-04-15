//! Api requests via yew FetchService

mod articles;
mod auth;
mod comments;
mod profiles;
mod requests;
mod tags;

pub use articles::Articles;
pub use auth::Auth;
pub use comments::Comments;
pub use profiles::Profiles;
pub use requests::{get_token, is_authenticated, limit, set_token, Requests};
pub use tags::Tags;
