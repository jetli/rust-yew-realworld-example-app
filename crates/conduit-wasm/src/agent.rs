#![allow(dead_code)]

use failure::{format_err, Error};
use serde::{Deserialize, Serialize};
use yew::callback::Callback;
use yew::format::{Json, Nothing, Text};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use crate::types::{ArticleListInfo, TagListInfo};

const API_ROOT: &'static str = "https://conduit.productionready.io/api";

struct Requests {
    fetch: FetchService,
}

impl Requests {
    fn new() -> Self {
        Self {
            fetch: FetchService::new(),
        }
    }

    fn builder<B, T>(&mut self, method: &str, url: String, body: B, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static,
        B: Into<Text>,
    {
        let url = format!("{}{}", API_ROOT, url);
        let handler = move |response: Response<Json<Result<T, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                callback.emit(Err(format_err!("{}: error getting data", meta.status)))
            }
        };
        let request = Request::builder().method(method).uri(url.as_str()).body(body).unwrap();
        self.fetch.fetch(request, handler.into())
    }

    fn delete<T>(&mut self, url: String, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static,
    {
        self.builder("DELETE", url, Nothing, callback)
    }

    fn get<T>(&mut self, url: String, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static,
    {
        self.builder("GET", url, Nothing, callback)
    }

    fn post<B, T>(&mut self, url: String, body: B, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static,
        B: Serialize,
    {
        let body: Text = Json(&body).into();
        self.builder("POST", url, body, callback)
    }

    fn put<B, T>(&mut self, url: String, body: B, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static,
        B: Serialize,
    {
        let body: Text = Json(&body).into();
        self.builder("PUT", url, body, callback)
    }
}

fn limit(count: u32, p: u32) -> String {
    let offset = if p > 0 { p * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}

pub struct Articles {
    requests: Requests,
}

impl Articles {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub fn all(
        &mut self,
        page: u32,
        callback: Callback<Result<ArticleListInfo, Error>>,
    ) -> FetchTask {
        self.requests
            .get::<ArticleListInfo>(format!("/articles?{}", limit(10, page)), callback)
    }

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
}

pub struct Tags {
    requests: Requests,
}

impl Tags {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub fn get_all(&mut self, callback: Callback<Result<TagListInfo, Error>>) -> FetchTask {
        self.requests.get::<TagListInfo>(format!("/tags"), callback)
    }
}
