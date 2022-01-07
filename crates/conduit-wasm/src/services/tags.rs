use super::request_get;
use crate::error::Error;
use crate::types::*;

/// Get all tags
pub async fn get_all() -> Result<TagListInfo, Error> {
    request_get::<TagListInfo>("/tags".to_string()).await
}
