use super::{request_delete, request_get, request_post};
use crate::error::Error;
use crate::types::*;

pub async fn follow(username: String) -> Result<ProfileInfoWrapper, Error> {
    request_post::<(), ProfileInfoWrapper>(format!("/profiles/{}/follow", username), ()).await
}

pub async fn unfollow(username: String) -> Result<ProfileInfoWrapper, Error> {
    request_delete::<ProfileInfoWrapper>(format!("/profiles/{}/follow", username)).await
}

pub async fn get(username: String) -> Result<ProfileInfoWrapper, Error> {
    request_get::<ProfileInfoWrapper>(format!("/profiles/{}", username)).await
}
