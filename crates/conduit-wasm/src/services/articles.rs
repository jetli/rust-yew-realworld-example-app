use super::{limit, request_delete, request_get, request_post, request_put};
use crate::error::Error;
use crate::types::*;

/// Get all articles
pub async fn all(page: u32) -> Result<ArticleListInfo, Error> {
    request_get::<ArticleListInfo>(format!("/articles?{}", limit(10, page))).await
}

/// Get articles filtered by author
pub async fn by_author(author: String, page: u32) -> Result<ArticleListInfo, Error> {
    request_get::<ArticleListInfo>(format!("/articles?author={}&{}", author, limit(10, page))).await
}

/// Get articles filtered by tag
pub async fn by_tag(tag: String, page: u32) -> Result<ArticleListInfo, Error> {
    request_get::<ArticleListInfo>(format!("/articles?tag={}&{}", tag, limit(10, page))).await
}

/// Delete an article
pub async fn del(slug: String) -> Result<DeleteWrapper, Error> {
    request_delete::<DeleteWrapper>(format!("/articles/{}", slug)).await
}

/// Favorite and article
pub async fn favorite(slug: String) -> Result<ArticleInfoWrapper, Error> {
    request_post::<(), ArticleInfoWrapper>(format!("/articles/{}/favorite", slug), ()).await
}

/// Unfavorite an article
pub async fn unfavorite(slug: String) -> Result<ArticleInfoWrapper, Error> {
    request_delete::<ArticleInfoWrapper>(format!("/articles/{}/favorite", slug)).await
}

/// Get articles favorited by an author
pub async fn favorited_by(author: String, page: u32) -> Result<ArticleListInfo, Error> {
    request_get::<ArticleListInfo>(format!(
        "/articles?favorited={}&{}",
        author,
        limit(10, page)
    ))
    .await
}

/// Get feed of articles
pub async fn feed() -> Result<ArticleListInfo, Error> {
    request_get::<ArticleListInfo>(format!("/articles/feed?{}", limit(10, 0))).await
}

/// Get an article
pub async fn get(slug: String) -> Result<ArticleInfoWrapper, Error> {
    request_get::<ArticleInfoWrapper>(format!("/articles/{}", slug)).await
}

/// Update an article
pub async fn update(
    slug: String,
    article: ArticleCreateUpdateInfoWrapper,
) -> Result<ArticleInfoWrapper, Error> {
    request_put::<ArticleCreateUpdateInfoWrapper, ArticleInfoWrapper>(
        format!("/articles/{}", slug),
        article,
    )
    .await
}

/// Create an article
pub async fn create(article: ArticleCreateUpdateInfoWrapper) -> Result<ArticleInfoWrapper, Error> {
    request_post::<ArticleCreateUpdateInfoWrapper, ArticleInfoWrapper>(
        "/articles".to_string(),
        article,
    )
    .await
}
