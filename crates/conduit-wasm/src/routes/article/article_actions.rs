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
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ArticleActions {
            articles: Articles::new(),
            response: link.callback(Msg::Response),
            task: None,
            props,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
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

    fn view(&self) -> Html {
        if self.props.can_modify {
            let onclick = self.link.callback(|_| Msg::DeleteArticle);
            html! {
                <span>
                    <RouterAnchor<AppRoute> route=AppRoute::Editor(self.props.slug.clone()) classes="btn btn-outline-secondary btn-sm" >
                        { "Edit Article" }
                    </RouterAnchor<AppRoute>>
                    { " " }
                    <button class="btn btn-outline-danger btn-sm" onclick=onclick >
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
