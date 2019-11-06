use failure::{format_err, Error};
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use crate::types::ArticleListInfo;

const API_ROOT: &'static str = "https://conduit.productionready.io/api";

pub struct Articles {
    fetch: FetchService,
}

impl Articles {
    pub fn new() -> Self {
        Self {
            fetch: FetchService::new(),
        }
    }
    pub fn all(&mut self, callback: Callback<Result<ArticleListInfo, Error>>) -> FetchTask {
        let url = format!("{}/articles", API_ROOT);
        let handler = move |response: Response<Json<Result<ArticleListInfo, Error>>>| {
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
