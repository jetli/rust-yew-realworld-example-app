use failure::{format_err, Error};
use serde::Deserialize;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use crate::types::ArticleListInfo;

const API_ROOT: &'static str = "https://conduit.productionready.io/api";

pub struct Requests {
    fetch: FetchService,
}

impl Requests {
    pub fn new() -> Self {
        Self {
            fetch: FetchService::new(),
        }
    }

    pub fn get<T>(&mut self, url: String, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static,
    {
        let url = format!("{}{}", API_ROOT, url);
        let handler = move |response: Response<Json<Result<T, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                callback.emit(Err(format_err!(
                    "{}: error getting article list",
                    meta.status
                )))
            }
        };
        let request = Request::get(url.as_str()).body(Nothing).unwrap();
        self.fetch.fetch(request, handler.into())
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
