use stdweb::web::event::IEvent;
use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

use crate::agent::Articles;
use crate::error::Error;
use crate::types::{ArticleInfo, ArticleInfoWrapper};

const FAVORITED_CLASS: &str = "btn btn-sm btn-primary";
const NOT_FAVORITED_CLASS: &str = "btn btn-sm btn-outline-primary";

/// Single article preview component used by article list.
pub struct ArticlePreview {
    articles: Articles,
    props: Props,
    response: Callback<Result<ArticleInfoWrapper, Error>>,
    task: Option<FetchTask>,
}

pub enum Msg {
    Request,
    Response(Result<ArticleInfoWrapper, Error>),
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub article: ArticleInfo,
}

impl Component for ArticlePreview {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        ArticlePreview {
            articles: Articles::new(),
            props,
            response: link.send_back(Msg::Response),
            task: None,
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

    fn view(&self) -> Html<Self> {
        let article = &self.props.article;
        let favorite_button_class = if article.favorited {
            FAVORITED_CLASS
        } else {
            NOT_FAVORITED_CLASS
        };

        html! {
            <div class="article-preview">
                <div class="article-meta">
                    <img src={ &article.author.image } />
                    <div class="info">
                        <RouterLink text={ &article.author.username } link={ format!("#/@{}", &article.author.username) } classes="author" />
                        <span class="date">
                            { &article.created_at }
                        </span>
                    </div>
                    <div class="pull-xs-right">
                        <button class=favorite_button_class onclick=|ev| { ev.prevent_default(); Msg::Request }>
                            <i class="ion-heart"></i> { article.favorites_count }
                        </button>
                    </div>
                </div>
                <h1><RouterLink text={ &article.title } link={ format!("#/article/{}", &article.slug) } classes="preview-link" /></h1>
                <p>{ &article.description }</p>
                <span><RouterLink text="Read more..." link={ format!("#/article/{}", &article.slug) } /></span>
                <ul class="tag-list">
                    {for article.tag_list.iter().map(|tag| {
                        html! {
                            <li class="tag-default tag-pill tag-outline" key={ &tag }>
                                { &tag }
                            </li>
                        }
                    })}
                </ul>
            </div>
        }
    }
}
