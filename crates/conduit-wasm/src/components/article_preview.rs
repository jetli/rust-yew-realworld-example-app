use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};
use yew_router::prelude::*;

use crate::error::Error;
use crate::routes::AppRoute;
use crate::services::Articles;
use crate::types::{ArticleInfo, ArticleInfoWrapper};

const FAVORITED_CLASS: &str = "btn btn-sm btn-primary";
const NOT_FAVORITED_CLASS: &str = "btn btn-sm btn-outline-primary";

/// Single article preview component used by article list.
pub struct ArticlePreview {
    articles: Articles,
    props: Props,
    response: Callback<Result<ArticleInfoWrapper, Error>>,
    task: Option<FetchTask>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Request,
    Response(Result<ArticleInfoWrapper, Error>),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub article: ArticleInfo,
}

impl Component for ArticlePreview {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ArticlePreview {
            articles: Articles::new(),
            props,
            response: link.callback(Msg::Response),
            task: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => {
                if self.props.article.favorited {
                    let task = self
                        .articles
                        .unfavorite(self.props.article.slug.clone(), self.response.clone());
                    self.task = Some(task);
                } else {
                    let task = self
                        .articles
                        .favorite(self.props.article.slug.clone(), self.response.clone());
                    self.task = Some(task);
                }
            }
            Msg::Response(Ok(article_info)) => {
                self.props.article = article_info.article;
                self.task = None;
            }
            Msg::Response(Err(_)) => {
                self.task = None;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let article = &self.props.article;
        let favorite_button_class = if article.favorited {
            FAVORITED_CLASS
        } else {
            NOT_FAVORITED_CLASS
        };
        let onclick = self.link.callback(|ev: MouseEvent| {
            ev.prevent_default();
            Msg::Request
        });

        html! {
            <div class="article-preview">
                <div class="article-meta">
                    <img src={ article.author.image.clone() } />
                    <div class="info">
                        <RouterAnchor<AppRoute>
                            route=AppRoute::Profile(article.author.username.clone())
                            classes="author" >
                            { &article.author.username }
                        </RouterAnchor<AppRoute>>
                        <span class="date">
                            { &article.created_at.format("%B %e, %Y") }
                        </span>
                    </div>
                    <div class="pull-xs-right">
                        <button class=favorite_button_class onclick=onclick>
                            <i class="ion-heart"></i> { article.favorites_count }
                        </button>
                    </div>
                </div>
                <h1>
                    <RouterAnchor<AppRoute>
                        route=AppRoute::Article(article.slug.clone())
                        classes="preview-link" >
                    { &article.title }
                    </RouterAnchor<AppRoute>>
                </h1>
                <p>{ &article.description }</p>
                <span>
                    <RouterAnchor<AppRoute>
                        route=AppRoute::Article(article.slug.clone())
                        classes="preview-link" >
                        { "Read more..." }
                    </RouterAnchor<AppRoute>>
                </span>
                <ul class="tag-list">
                    {for article.tag_list.iter().map(|tag| {
                        html! {
                            <li class="tag-default tag-pill tag-outline" key={ (&tag).to_string() }>
                                { &tag }
                            </li>
                        }
                    })}
                </ul>
            </div>
        }
    }
}
