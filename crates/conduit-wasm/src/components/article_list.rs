use log::info;
use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

//use super::article_preview::ArticlePreview;
use crate::agent::Articles;
use crate::error::Error;
use crate::types::{ArticleInfo, ArticleListInfo};

pub struct ArticleList {
    articles: Articles,
    article_list: Option<ArticleListInfo>,
    article_list_callback: Callback<Result<ArticleListInfo, Error>>,
    article_list_task: Option<FetchTask>,
}

pub enum Msg {
    ArticleListReady(Result<ArticleListInfo, Error>),
}

impl Component for ArticleList {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        ArticleList {
            articles: Articles::new(),
            article_list: None,
            article_list_callback: link.send_back(Msg::ArticleListReady),
            article_list_task: None,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        let task = self.articles.all(0, self.article_list_callback.clone());
        self.article_list_task = Some(task);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ArticleListReady(Ok(article_list)) => {
                self.article_list = Some(article_list);
            }
            Msg::ArticleListReady(Err(err)) => {
                // Can't load data
                info!("{:?}", err);
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        if let Some(article_list) = &self.article_list {
            if !article_list.articles.is_empty() {
                html! {
                    <>
                        {for article_list.articles.iter().map(|article| {
                            self.article_preview(&article)
                        })}
                    </>
                }
            } else {
                html! {
                    <div class="article-preview">{ "No articles are here... yet." }</div>
                }
            }
        } else {
            html! {
                <div class="article-preview">{ "Loading..." }</div>
            }
        }
    }
}

impl ArticleList {
    fn article_preview(&self, article: &ArticleInfo) -> Html<Self> {
        html! {
            <div class="article-preview">
                <div class="article-meta">
                    <img src={ &article.author.image } />
                    <div class="info">
                        <RouterLink text={ &article.author.username } link={ format!("/#/@{}", &article.author.username) } classes="author" />
                        <span class="date">
                            { &article.created_at }
                        </span>
                    </div>
                    <div class="pull-xs-right">
                        <button class="btn btn-sm btn-outline-primary">
                            <i class="ion-heart"></i> { article.favorites_count }
                        </button>
                    </div>
                </div>
                <h1><RouterLink text={ &article.title } link={ format!("/#/article/{}", &article.slug) } classes="preview-link" /></h1>
                <p>{ &article.description }</p>
                <span><RouterLink text="Read more..." link={ format!("/#/article/{}", &article.slug) } /></span>
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
