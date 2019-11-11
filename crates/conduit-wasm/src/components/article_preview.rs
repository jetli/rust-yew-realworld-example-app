use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

use crate::types::ArticleInfo;

//const FAVORITED_CLASS: &str = "btn btn-sm btn-primary";
const NOT_FAVORITED_CLASS: &str = "btn btn-sm btn-outline-primary";

pub struct ArticlePreview {
    #[allow(dead_code)]
    props: Props,
}

pub enum Msg {}

#[derive(Properties)]
pub struct Props {
    pub article: ArticleInfo,
}

impl Component for ArticlePreview {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ArticlePreview { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="article-preview">
                <div class="article-meta">
                    <img src="{article.author.image}" />
                    <div class="info">
                        <RouterLink text="{article.author.username}" link="/@{article.author.username}" classes="author" />
                        <span class="date">
                            { "{article.createdAt}" }
                        </span>
                    </div>
                    <div class="pull-xs-right">
                        <button class={ NOT_FAVORITED_CLASS }>
                            <i class="ion-heart"></i> { "{article.favoritesCount}" }
                        </button>
                    </div>
                </div>


                <h1><RouterLink text="{article.title}" link="/article/{article.slug}" classes="preview-link" /></h1>
                <p>{ "{article.description}" }</p>
                <span><RouterLink text="Read more..." link="/article/{article.slug}" /></span>
                <ul class="tag-list">
                    <li class="tag-default tag-pill tag-outline" key="{tag}">
                        { "{tag}" }
                    </li>
                </ul>
            </div>
        }
    }
}
