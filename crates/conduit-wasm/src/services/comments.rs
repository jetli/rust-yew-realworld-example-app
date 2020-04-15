use yew::callback::Callback;
use yew::services::fetch::FetchTask;

use super::Requests;
use crate::error::Error;
use crate::types::*;

/// Apis for comments
#[derive(Default, Debug)]
pub struct Comments {
    requests: Requests,
}

impl Comments {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub fn create(
        &mut self,
        slug: String,
        comment: CommentCreateInfoWrapper,
        callback: Callback<Result<CommentInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests
            .post::<CommentCreateInfoWrapper, CommentInfoWrapper>(
                format!("/articles/{}/comments", slug),
                comment,
                callback,
            )
    }

    pub fn delete(
        &mut self,
        slug: String,
        comment_id: u32,
        callback: Callback<Result<DeleteWrapper, Error>>,
    ) -> FetchTask {
        self.requests.delete::<DeleteWrapper>(
            format!("/articles/{}/comments/{}", slug, comment_id),
            callback,
        )
    }

    pub fn for_article(
        &mut self,
        slug: String,
        callback: Callback<Result<CommentListInfo, Error>>,
    ) -> FetchTask {
        self.requests
            .get::<CommentListInfo>(format!("/articles/{}/comments", slug), callback)
    }
}
