use stdweb::web::event::{IEvent, IKeyboardEvent};
use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, Html, Properties,
    ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::agent::Articles;
use crate::components::list_errors::ListErrors;
use crate::error::Error;
use crate::routes::AppRoute;
use crate::types::{ArticleCreateUpdateInfo, ArticleCreateUpdateInfoWrapper, ArticleInfoWrapper};

/// Create or update an article
pub struct Editor {
    articles: Articles,
    error: Option<Error>,
    request: ArticleCreateUpdateInfo,
    tag_input: String,
    response: Callback<Result<ArticleInfoWrapper, Error>>,
    loaded: Callback<Result<ArticleInfoWrapper, Error>>,
    task: Option<FetchTask>,
    props: Props,
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

#[derive(Properties)]
pub struct Props {
    pub slug: Option<String>,
}

pub enum Msg {
    Request,
    Response(Result<ArticleInfoWrapper, Error>),
    Loaded(Result<ArticleInfoWrapper, Error>),
    Ignore,
    UpdateTitle(String),
    UpdateDescription(String),
    UpdateBody(String),
    UpdateTagInput(String),
    AddTag,
    RemoveTag(String),
}

impl Component for Editor {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        Editor {
            articles: Articles::new(),
            error: None,
            request: ArticleCreateUpdateInfo::default(),
            tag_input: String::default(),
            response: link.send_back(Msg::Response),
            loaded: link.send_back(Msg::Loaded),
            task: None,
            props,
            router_agent: RouteAgent::bridge(link.send_back(|_| Msg::Ignore)),
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        if let Some(slug) = &self.props.slug {
            self.task = Some(self.articles.get(slug.clone(), self.loaded.clone()));
        }
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddTag => {
                // Add a new tag
                if let Some(tag_list) = &mut self.request.tag_list {
                    if !tag_list.contains(&self.tag_input) {
                        tag_list.push(self.tag_input.clone());
                    }
                } else {
                    self.request.tag_list = Some(vec![self.tag_input.clone()]);
                }
                // Clear tag input
                self.tag_input = String::default();
            }
            Msg::Ignore => {}
            Msg::RemoveTag(tag) => {
                // Remove a tag
                if let Some(tag_list) = &mut self.request.tag_list {
                    tag_list.retain(|t| t != &tag);
                }
            }
            Msg::Request => {
                let request = ArticleCreateUpdateInfoWrapper {
                    article: self.request.clone(),
                };
                // Update or create an article
                if let Some(slug) = &self.props.slug {
                    self.task = Some(self.articles.update(
                        slug.clone(),
                        request.clone(),
                        self.response.clone(),
                    ));
                } else {
                    self.task = Some(self.articles.create(request.clone(), self.response.clone()));
                }
            }
            Msg::Response(Ok(article_info)) => {
                self.error = None;
                self.task = None;
                self.router_agent.send(ChangeRoute(
                    AppRoute::Article(article_info.article.slug.clone()).into(),
                ));
            }
            Msg::Response(Err(err)) => {
                self.error = Some(err);
                self.task = None;
            }
            Msg::Loaded(Ok(article_info)) => {
                self.error = None;
                self.task = None;
                // Load the article to editor form
                self.request = ArticleCreateUpdateInfo {
                    title: article_info.article.title,
                    description: article_info.article.description,
                    body: article_info.article.body,
                    tag_list: Some(article_info.article.tag_list),
                };
            }
            Msg::Loaded(Err(err)) => {
                self.error = Some(err);
                self.task = None;
            }
            Msg::UpdateBody(body) => {
                self.request.body = body;
            }
            Msg::UpdateDescription(description) => {
                self.request.description = description;
            }
            Msg::UpdateTagInput(tag_input) => {
                self.tag_input = tag_input;
            }
            Msg::UpdateTitle(title) => {
                self.request.title = title;
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="editor-page">
                <div class="container page">
                    <div class="row">
                        <div class="col-md-10 offset-md-1 col-xs-12">
                            <ListErrors error=&self.error />
                            <form onsubmit=|ev| { ev.prevent_default(); Msg::Request }>
                                <fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="text"
                                            placeholder="Article Title"
                                            value={ &self.request.title }
                                            oninput=|ev| Msg::UpdateTitle(ev.value) />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control"
                                            type="text"
                                            placeholder="What's this article about?"
                                            value={ &self.request.description }
                                            oninput=|ev| Msg::UpdateDescription(ev.value) />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <textarea
                                            class="form-control"
                                            rows="8"
                                            placeholder="Write your article (in markdown)"
                                            value={ &self.request.body}
                                            oninput=|ev| Msg::UpdateBody(ev.value) >
                                        </textarea>
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control"
                                            type="text"
                                            placeholder="Enter tags"
                                            value={ &self.tag_input }
                                            oninput=|ev| Msg::UpdateTagInput(ev.value)
                                            onkeypress=|ev| {
                                                // Prevent submit the form when press Enter
                                                if ev.code() == "Enter" {
                                                    ev.prevent_default();
                                                }
                                                Msg::Ignore
                                            }
                                            onkeyup=|ev| {
                                                // Add a new tag when press Enter
                                                if ev.code() == "Enter" {
                                                    ev.prevent_default();
                                                    Msg::AddTag
                                                } else {
                                                    Msg::Ignore
                                                }} />
                                        <div class="tag-list">
                                            {
                                                if let Some(tag_list) = &self.request.tag_list {
                                                    html! {for tag_list.iter().map(|tag| {
                                                        let tag_to_remove = tag.clone();
                                                        html! {
                                                            <span class="tag-default tag-pill">
                                                                <i class="ion-close-round"
                                                                    onclick=|ev| Msg::RemoveTag(tag_to_remove.clone())>
                                                                </i>
                                                                { &tag }
                                                            </span>
                                                        }
                                                    })}
                                                } else {
                                                    html! {}
                                                }
                                            }
                                        </div>
                                    </fieldset>
                                    <button
                                        class="btn btn-lg pull-xs-right btn-primary"
                                        type="submit"
                                        disabled=false>
                                        { "Publish Article" }
                                    </button>
                                </fieldset>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
