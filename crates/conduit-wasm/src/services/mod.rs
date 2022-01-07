//! Api requests via yew FetchService

pub mod articles;
pub mod auth;
pub mod comments;
pub mod profiles;
pub mod requests;
pub mod tags;

pub use requests::{
    get_token, limit, request_delete, request_get, request_post, request_put, set_token,
};
