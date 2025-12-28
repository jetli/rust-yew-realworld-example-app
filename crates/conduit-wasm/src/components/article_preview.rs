use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;
use crate::services::articles::*;
use crate::types::ArticleInfo;

const FAVORITED_CLASS: &str = "btn btn-sm btn-primary";
const NOT_FAVORITED_CLASS: &str = "btn btn-sm btn-outline-primary";

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub article: ArticleInfo,
}

/// Single article preview component used by article list.
#[function_component]
pub fn ArticlePreview(props: &Props) -> Html {
    let article = use_state(|| props.article.clone());
    let article_favorite = {
        let article = article.clone();
        use_async(async move {
            if article.favorited {
                unfavorite(article.slug.clone()).await
            } else {
                favorite(article.slug.clone()).await
            }
        })
    };

    {
        let article = article.clone();
        use_effect_with(props.clone(), move |props| {
            article.set(props.article.clone());
        })
    }

    {
        let article = article.clone();
        use_effect_with(article_favorite.clone(), move |article_favorite| {
            if let Some(article_info) = &article_favorite.data {
                article.set(article_info.article.clone());
            }
            || ()
        });
    }

    let favorite_button_class = if article.favorited {
        FAVORITED_CLASS
    } else {
        NOT_FAVORITED_CLASS
    };
    let onclick = {
        Callback::from(move |ev: MouseEvent| {
            ev.prevent_default();
            article_favorite.run();
        })
    };

    html! {
        <div class="article-preview">
            <div class="article-meta">
                <img src={article.author.image.clone()} />
                <div class="info">
                    <Link<AppRoute>
                        to={AppRoute::Profile { username: article.author.username.clone() }}
                        classes="author" >
                        { &article.author.username }
                    </Link<AppRoute>>
                    <span class="date">
                        { &article.created_at.format("%B %e, %Y").to_string() }
                    </span>
                </div>
                <div class="pull-xs-right">
                    <button class={favorite_button_class} onclick={onclick}>
                        <i class="ion-heart"></i> { article.favorites_count }
                    </button>
                </div>
            </div>
            <h1>
                <Link<AppRoute>
                    to={AppRoute::Article { slug: article.slug.clone() }}
                    classes="preview-link" >
                { &article.title }
                </Link<AppRoute>>
            </h1>
            <p>{ &article.description }</p>
            <span>
                <Link<AppRoute>
                    to={AppRoute::Article { slug: article.slug.clone() }}
                    classes="preview-link" >
                    { "Read more..." }
                </Link<AppRoute>>
            </span>
            <ul class="tag-list">
                {for article.tag_list.iter().map(|tag| {
                    html! {
                        <li class="tag-default tag-pill tag-outline" key={ (&tag).to_string() }>
                            { tag }
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}
