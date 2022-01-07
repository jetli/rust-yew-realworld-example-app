use super::{request_delete, request_get, request_post};
use crate::error::Error;
use crate::types::*;

pub async fn create(
    slug: String,
    comment: CommentCreateInfoWrapper,
) -> Result<CommentInfoWrapper, Error> {
    request_post::<CommentCreateInfoWrapper, CommentInfoWrapper>(
        format!("/articles/{}/comments", slug),
        comment,
    )
    .await
}

pub async fn delete(slug: String, comment_id: u32) -> Result<DeleteWrapper, Error> {
    request_delete::<DeleteWrapper>(format!("/articles/{}/comments/{}", slug, comment_id)).await
}

pub async fn for_article(slug: String) -> Result<CommentListInfo, Error> {
    request_get::<CommentListInfo>(format!("/articles/{}/comments", slug)).await
}
