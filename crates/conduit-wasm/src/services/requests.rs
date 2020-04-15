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
use crate::types::ErrorInfo;

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
pub struct Requests {
    fetch: FetchService,
}

impl Requests {
    pub fn new() -> Self {
        Self {
            fetch: FetchService::new(),
        }
    }

    /// build all kinds of http request: post/get/delete etc.
    pub fn builder<B, T>(
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
    pub fn delete<T>(&mut self, url: String, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
    {
        self.builder("DELETE", url, Nothing, callback)
    }

    /// Get request
    pub fn get<T>(&mut self, url: String, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
    {
        self.builder("GET", url, Nothing, callback)
    }

    /// Post request with a body
    pub fn post<B, T>(
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
    pub fn put<B, T>(
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
        self.builder("PUT", url, body, callback)
    }
}

/// Set limit for pagination
pub fn limit(count: u32, p: u32) -> String {
    let offset = if p > 0 { p * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}
