//! Api requests via yew FetchService

#![allow(dead_code)]

use dotenv_codegen::dotenv;
use lazy_static::lazy_static;
use log::debug;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use serde_json;
use yew::callback::Callback;
use yew::format::{Json, Nothing, Text};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::storage::{Area, StorageService};

use crate::error::Error;
use crate::types::*;

const API_ROOT: &str = dotenv!("API_ROOT");
const TOKEN_KEY: &str = "yew.token";

lazy_static! {
    /// Jwt token read from local storage.
    pub static ref TOKEN: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(token) = storage.restore(TOKEN_KEY) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };
}

/// Set jwt token to local storage.
pub fn set_token(token: Option<String>) {
    let mut storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
    if let Some(t) = token.clone() {
        storage.store(TOKEN_KEY, Ok(t));
    } else {
        storage.remove(TOKEN_KEY);
    }
    let mut token_lock = TOKEN.write();
    *token_lock = token;
}

/// Get jwt token from lazy static.
pub fn get_token() -> Option<String> {
    let token_lock = TOKEN.read();
    token_lock.clone()
}

/// Check if current user is authenticated.
pub fn is_authenticated() -> bool {
    get_token().is_some()
}

/// Http request
#[derive(Default, Debug)]
struct Requests {
    fetch: FetchService,
}

impl Requests {
    fn new() -> Self {
        Self {
            fetch: FetchService::new(),
        }
    }

    /// build all kinds of http request: post/get/delete etc.
    fn builder<B, T>(
        &mut self,
        method: &str,
        url: String,
        body: B,
        callback: Callback<Result<T, Error>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
        B: Into<Text> + std::fmt::Debug,
    {
        let handler = move |response: Response<Text>| {
            if let (meta, Ok(data)) = response.into_parts() {
                debug!("Response: {:?}", data);
                if meta.status.is_success() {
                    let data: Result<T, _> = serde_json::from_str(&data);
                    if let Ok(data) = data {
                        callback.emit(Ok(data))
                    } else {
                        callback.emit(Err(Error::DeserializeError))
                    }
                } else {
                    match meta.status.as_u16() {
                        401 => callback.emit(Err(Error::Unauthorized)),
                        403 => callback.emit(Err(Error::Forbidden)),
                        404 => callback.emit(Err(Error::NotFound)),
                        500 => callback.emit(Err(Error::InternalServerError)),
                        422 => {
                            let data: Result<ErrorInfo, _> = serde_json::from_str(&data);
                            if let Ok(data) = data {
                                callback.emit(Err(Error::UnprocessableEntity(data)))
                            } else {
                                callback.emit(Err(Error::DeserializeError))
                            }
                        }
                        _ => callback.emit(Err(Error::RequestError)),
                    }
                }
            } else {
                callback.emit(Err(Error::RequestError))
            }
        };

        let url = format!("{}{}", API_ROOT, url);
        let mut builder = Request::builder()
            .method(method)
            .uri(url.as_str())
            .header("Content-Type", "application/json");
        if let Some(token) = get_token() {
            builder = builder.header("Authorization", format!("Token {}", token));
        }
        let request = builder.body(body).unwrap();
        debug!("Request: {:?}", request);

        self.fetch.fetch(request, handler.into()).unwrap()
    }

    /// Delete request
    fn delete<T>(&mut self, url: String, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
    {
        self.builder("DELETE", url, Nothing, callback)
    }

    /// Get request
    fn get<T>(&mut self, url: String, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
    {
        self.builder("GET", url, Nothing, callback)
    }

    /// Post request with a body
    fn post<B, T>(
        &mut self,
        url: String,
        body: B,
        callback: Callback<Result<T, Error>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
        B: Serialize,
    {
        let body: Text = Json(&body).into();
        self.builder("POST", url, body, callback)
    }

    /// Put request with a body
    fn put<B, T>(&mut self, url: String, body: B, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
        B: Serialize,
    {
        let body: Text = Json(&body).into();
        self.builder("PUT", url, body, callback)
    }
}

/// Set limit for pagination
fn limit(count: u32, p: u32) -> String {
    let offset = if p > 0 { p * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}

/// Apis for articles
#[derive(Default, Debug)]
pub struct Articles {
    requests: Requests,
}

impl Articles {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    /// Get all articles
    pub fn all(
        &mut self,
        page: u32,
        callback: Callback<Result<ArticleListInfo, Error>>,
    ) -> FetchTask {
        self.requests
            .get::<ArticleListInfo>(format!("/articles?{}", limit(10, page)), callback)
    }

    /// Get articles filtered by author
    pub fn by_author(
        &mut self,
        author: String,
        page: u32,
        callback: Callback<Result<ArticleListInfo, Error>>,
    ) -> FetchTask {
        self.requests.get::<ArticleListInfo>(
            format!("/articles?author={}&{}", author, limit(10, page)),
            callback,
        )
    }

    /// Get articles filtered by tag
    pub fn by_tag(
        &mut self,
        tag: String,
        page: u32,
        callback: Callback<Result<ArticleListInfo, Error>>,
    ) -> FetchTask {
        self.requests.get::<ArticleListInfo>(
            format!("/articles?tag={}&{}", tag, limit(10, page)),
            callback,
        )
    }

    /// Delete an article
    pub fn del(
        &mut self,
        slug: String,
        callback: Callback<Result<DeleteWrapper, Error>>,
    ) -> FetchTask {
        self.requests
            .delete::<DeleteWrapper>(format!("/articles/{}", slug), callback)
    }

    /// Favorite and article
    pub fn favorite(
        &mut self,
        slug: String,
        callback: Callback<Result<ArticleInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests.post::<(), ArticleInfoWrapper>(
            format!("/articles/{}/favorite", slug),
            (),
            callback,
        )
    }

    /// Unfavorite an article
    pub fn unfavorite(
        &mut self,
        slug: String,
        callback: Callback<Result<ArticleInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests
            .delete::<ArticleInfoWrapper>(format!("/articles/{}/favorite", slug), callback)
    }

    /// Get articles favorited by an author
    pub fn favorited_by(
        &mut self,
        author: String,
        page: u32,
        callback: Callback<Result<ArticleListInfo, Error>>,
    ) -> FetchTask {
        self.requests.get::<ArticleListInfo>(
            format!("/articles?favorited={}&{}", author, limit(10, page)),
            callback,
        )
    }

    /// Get feed of articles
    pub fn feed(&mut self, callback: Callback<Result<ArticleListInfo, Error>>) -> FetchTask {
        self.requests
            .get::<ArticleListInfo>(format!("/articles/feed?{}", limit(10, 0)), callback)
    }

    /// Get an article
    pub fn get(
        &mut self,
        slug: String,
        callback: Callback<Result<ArticleInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests
            .get::<ArticleInfoWrapper>(format!("/articles/{}", slug), callback)
    }

    /// Update an article
    pub fn update(
        &mut self,
        slug: String,
        article: ArticleCreateUpdateInfoWrapper,
        callback: Callback<Result<ArticleInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests
            .put::<ArticleCreateUpdateInfoWrapper, ArticleInfoWrapper>(
                format!("/articles/{}", slug),
                article,
                callback,
            )
    }

    /// Create an article
    pub fn create(
        &mut self,
        article: ArticleCreateUpdateInfoWrapper,
        callback: Callback<Result<ArticleInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests
            .post::<ArticleCreateUpdateInfoWrapper, ArticleInfoWrapper>(
                "/articles".to_string(),
                article,
                callback,
            )
    }
}

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

/// Apis for tags
#[derive(Default, Debug)]
pub struct Tags {
    requests: Requests,
}

impl Tags {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    /// Get all tags
    pub fn get_all(&mut self, callback: Callback<Result<TagListInfo, Error>>) -> FetchTask {
        self.requests
            .get::<TagListInfo>("/tags".to_string(), callback)
    }
}

/// Apis for authentication
#[derive(Default, Debug)]
pub struct Auth {
    requests: Requests,
}

impl Auth {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    /// Get current user info
    pub fn current(&mut self, callback: Callback<Result<UserInfoWrapper, Error>>) -> FetchTask {
        self.requests
            .get::<UserInfoWrapper>("/user".to_string(), callback)
    }

    /// Login a user
    pub fn login(
        &mut self,
        login_info: LoginInfoWrapper,
        callback: Callback<Result<UserInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests.post::<LoginInfoWrapper, UserInfoWrapper>(
            "/users/login".to_string(),
            login_info,
            callback,
        )
    }

    /// Register a new user
    pub fn register(
        &mut self,
        register_info: RegisterInfoWrapper,
        callback: Callback<Result<UserInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests.post::<RegisterInfoWrapper, UserInfoWrapper>(
            "/users".to_string(),
            register_info,
            callback,
        )
    }

    /// Save info of current user
    pub fn save(
        &mut self,
        user_update_info: UserUpdateInfoWrapper,
        callback: Callback<Result<UserInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests.put::<UserUpdateInfoWrapper, UserInfoWrapper>(
            "/user".to_string(),
            user_update_info,
            callback,
        )
    }
}

/// Apis for profile
#[derive(Default, Debug)]
pub struct Profiles {
    requests: Requests,
}

impl Profiles {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub fn follow(
        &mut self,
        username: String,
        callback: Callback<Result<ProfileInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests.post::<(), ProfileInfoWrapper>(
            format!("/profiles/{}/follow", username),
            (),
            callback,
        )
    }

    pub fn unfollow(
        &mut self,
        username: String,
        callback: Callback<Result<ProfileInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests
            .delete::<ProfileInfoWrapper>(format!("/profiles/{}/follow", username), callback)
    }

    pub fn get(
        &mut self,
        username: String,
        callback: Callback<Result<ProfileInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests
            .get::<ProfileInfoWrapper>(format!("/profiles/{}", username), callback)
    }
}
