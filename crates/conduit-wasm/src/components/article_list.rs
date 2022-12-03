use yew::prelude::*;
use yew_hooks::prelude::*;

use super::article_preview::ArticlePreview;
use super::list_pagination::ListPagination;
use crate::services::articles::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub filter: ArticleListFilter,
}

/// Filters for article list
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArticleListFilter {
    All,
    ByAuthor(String),
    ByTag(String),
    FavoritedBy(String),
    Feed,
}

/// List of articles component
#[function_component(ArticleList)]
pub fn article_list(props: &Props) -> Html {
    let current_page = use_state(|| 0u32);
    let article_list = {
        let filter = props.filter.clone();
        let current_page = current_page.clone();

        use_async(async move {
            match filter {
                ArticleListFilter::All => all(*current_page).await,
                ArticleListFilter::ByAuthor(author) => by_author(author, *current_page).await,
                ArticleListFilter::ByTag(tag) => by_tag(tag, *current_page).await,
                ArticleListFilter::FavoritedBy(author) => favorited_by(author, *current_page).await,
                ArticleListFilter::Feed => feed().await,
            }
        })
    };

    {
        let current_page = current_page.clone();
        use_effect_with_deps(
            move |_| {
                // Reset to first page
                current_page.set(0);
                || ()
            },
            props.filter.clone(),
        );
    }

    {
        let article_list = article_list.clone();
        use_effect_with_deps(
            move |_| {
                article_list.run();
                || ()
            },
            (props.filter.clone(), *current_page),
        );
    }

    let callback = {
        let current_page = current_page.clone();
        use_callback(move |page, _| {
            current_page.set(page);
        }, ())
    };

    if let Some(article_list) = &article_list.data {
        if !article_list.articles.is_empty() {
            html! {
                <>
                    {for article_list.articles.iter().map(|article| {
                        html! { <ArticlePreview article={article.clone()} /> }
                    })}
                    <ListPagination
                        total_count={article_list.articles_count}
                        current_page={*current_page}
                        callback={callback} />
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
