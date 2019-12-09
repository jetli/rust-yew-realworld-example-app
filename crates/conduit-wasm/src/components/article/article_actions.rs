use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, Html, Properties,
    ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::agent::Articles;
use crate::error::Error;
use crate::routes::AppRoute;
use crate::types::DeleteWrapper;

/// Article actions component to edit or delete an article.
pub struct ArticleActions {
    articles: Articles,
    response: Callback<Result<DeleteWrapper, Error>>,
    task: Option<FetchTask>,
    props: Props,
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub slug: String,
    #[props(required)]
    pub can_modify: bool,
}

pub enum Msg {
    DeleteArticle,
    Response(Result<DeleteWrapper, Error>),
    Ignore,
}

impl Component for ArticleActions {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        ArticleActions {
            articles: Articles::new(),
            response: link.send_back(Msg::Response),
            task: None,
            props,
            router_agent: RouteAgent::bridge(link.send_back(|_| Msg::Ignore)),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DeleteArticle => {
                self.task = Some(
                    self.articles
                        .del(self.props.slug.clone(), self.response.clone()),
                );
            }
            Msg::Response(Ok(_)) => {
                self.task = None;
                self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
            }
            Msg::Response(Err(_)) => {
                self.task = None;
            }
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html<Self> {
        if self.props.can_modify {
            html! {
                <span>
                    <RouterLink text={ "Edit Article" } link={ format!("#/editor/{}", &self.props.slug) } classes="btn btn-outline-secondary btn-sm" />
                    { " " }
                    <button class="btn btn-outline-danger btn-sm" onclick=|_| Msg::DeleteArticle >
                        <i class="ion-trash-a"></i> { "Delete Article" }
                    </button>
                </span>
            }
        } else {
            html! {
                <span>
                </span>
            }
        }
    }
}
