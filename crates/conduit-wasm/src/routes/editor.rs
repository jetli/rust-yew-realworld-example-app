use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, FocusEvent, Html, InputData,
    KeyboardEvent, Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::components::list_errors::ListErrors;
use crate::error::Error;
use crate::routes::AppRoute;
use crate::services::Articles;
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
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Editor {
            articles: Articles::new(),
            error: None,
            request: ArticleCreateUpdateInfo::default(),
            tag_input: String::default(),
            response: link.callback(Msg::Response),
            loaded: link.callback(Msg::Loaded),
            task: None,
            props,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(slug) = &self.props.slug {
                self.task = Some(self.articles.get(slug.clone(), self.loaded.clone()));
            }
        }
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
                        request,
                        self.response.clone(),
                    ));
                } else {
                    self.task = Some(self.articles.create(request, self.response.clone()));
                }
            }
            Msg::Response(Ok(article_info)) => {
                self.error = None;
                self.task = None;
                self.router_agent.send(ChangeRoute(
                    AppRoute::Article(article_info.article.slug).into(),
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::Request
        });
        let oninput_title = self
            .link
            .callback(|ev: InputData| Msg::UpdateTitle(ev.value));
        let oninput_description = self
            .link
            .callback(|ev: InputData| Msg::UpdateDescription(ev.value));
        let oninput_body = self
            .link
            .callback(|ev: InputData| Msg::UpdateBody(ev.value));
        let oninput_tag = self
            .link
            .callback(|ev: InputData| Msg::UpdateTagInput(ev.value));
        let onkeypress = self.link.callback(|ev: KeyboardEvent| {
            // Prevent submit the form when press Enter
            if ev.code() == "Enter" {
                ev.prevent_default();
            }
            Msg::Ignore
        });
        let onkeyup = self.link.callback(|ev: KeyboardEvent| {
            // Add a new tag when press Enter
            if ev.code() == "Enter" {
                ev.prevent_default();
                Msg::AddTag
            } else {
                Msg::Ignore
            }
        });

        html! {
            <div class="editor-page">
                <div class="container page">
                    <div class="row">
                        <div class="col-md-10 offset-md-1 col-xs-12">
                            <ListErrors error=self.error.clone() />
                            <form onsubmit=onsubmit>
                                <fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="text"
                                            placeholder="Article Title"
                                            value={ self.request.title.clone() }
                                            oninput=oninput_title />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control"
                                            type="text"
                                            placeholder="What's this article about?"
                                            value={ self.request.description.clone() }
                                            oninput=oninput_description />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <textarea
                                            class="form-control"
                                            rows="8"
                                            placeholder="Write your article (in markdown)"
                                            value={ self.request.body.clone()}
                                            oninput=oninput_body >
                                        </textarea>
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control"
                                            type="text"
                                            placeholder="Enter tags"
                                            value={ self.tag_input.clone() }
                                            oninput=oninput_tag
                                            onkeypress=onkeypress
                                            onkeyup=onkeyup />
                                        <div class="tag-list">
                                            {
                                                if let Some(tag_list) = &self.request.tag_list {
                                                    html! {for tag_list.iter().map(|tag| {
                                                        let tag_to_remove = tag.clone();
                                                        let onclick_remove = self.link.callback(move |_ev| Msg::RemoveTag(tag_to_remove.to_string()));
                                                        html! {
                                                            <span class="tag-default tag-pill">
                                                                <i class="ion-close-round"
                                                                    onclick=onclick_remove>
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
