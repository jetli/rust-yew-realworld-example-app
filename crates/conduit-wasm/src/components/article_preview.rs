use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

use crate::types::ArticleInfo;

const FAVORITED_CLASS: &str = "btn btn-sm btn-primary";
const NOT_FAVORITED_CLASS: &str = "btn btn-sm btn-outline-primary";

/// Single article preview component used by article list.
pub struct ArticlePreview {
    props: Props,
}

pub enum Msg {}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
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
                        <button class=favorite_button_class>
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
