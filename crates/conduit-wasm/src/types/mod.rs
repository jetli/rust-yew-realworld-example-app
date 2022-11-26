//! Common types

mod articles;
mod auth;
mod comments;
mod profiles;
mod tags;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use articles::{
    ArticleCreateUpdateInfo, ArticleCreateUpdateInfoWrapper, ArticleInfo, ArticleInfoWrapper,
    ArticleListInfo,
};
pub use auth::{
    LoginInfo, LoginInfoWrapper, RegisterInfo, RegisterInfoWrapper, UserInfo, UserInfoWrapper,
    UserUpdateInfo, UserUpdateInfoWrapper,
};
pub use comments::{
    CommentCreateInfo, CommentCreateInfoWrapper, CommentInfo, CommentInfoWrapper, CommentListInfo,
};
pub use profiles::{ProfileInfo, ProfileInfoWrapper};
pub use tags::TagListInfo;

/// Conduit api error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

pub type DeleteWrapper = HashMap<(), ()>;
