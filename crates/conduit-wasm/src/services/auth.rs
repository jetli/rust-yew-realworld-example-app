use super::{request_get, request_post, request_put};
use crate::error::Error;
use crate::types::*;

/// Get current user info
pub async fn current() -> Result<UserInfoWrapper, Error> {
    request_get::<UserInfoWrapper>("/user".to_string()).await
}

/// Login a user
pub async fn login(login_info: LoginInfoWrapper) -> Result<UserInfoWrapper, Error> {
    request_post::<LoginInfoWrapper, UserInfoWrapper>("/users/login".to_string(), login_info).await
}

/// Register a new user
pub async fn register(register_info: RegisterInfoWrapper) -> Result<UserInfoWrapper, Error> {
    request_post::<RegisterInfoWrapper, UserInfoWrapper>("/users".to_string(), register_info).await
}

/// Save info of current user
pub async fn save(user_update_info: UserUpdateInfoWrapper) -> Result<UserInfoWrapper, Error> {
    request_put::<UserUpdateInfoWrapper, UserInfoWrapper>("/user".to_string(), user_update_info)
        .await
}
