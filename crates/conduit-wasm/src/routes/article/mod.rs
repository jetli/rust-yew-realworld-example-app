mod article_actions;
mod article_meta;
mod comment;
mod comment_input;
mod comment_list;
mod delete_button;

use web_sys::Node;

use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::hooks::use_user_context;
use crate::services::articles::*;
use article_meta::ArticleMeta;
use comment_list::CommentList;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub slug: String,
}

/// Article detail page
#[function_component(Article)]
pub fn article(props: &Props) -> Html {
    let article = {
        let slug = props.slug.clone();
        use_async_with_options(
            async move { get(slug).await },
            UseAsyncOptions::enable_auto(),
        )
    };
    let user_ctx = use_user_context();

    if let Some(article) = &article.data {
        let article = &article.article;
        let can_modify =
            user_ctx.is_authenticated() && user_ctx.username == article.author.username;
        let created_at = article.created_at.format("%B %e, %Y").to_string();

        html! {
            <div class="article-page">
                <div class="banner">
                    <div class="container">
                        <h1>{&article.title}</h1>
                        <ArticleMeta
                            slug={article.slug.clone()}
                            author={article.author.clone()}
                            can_modify={can_modify}
                            created_at={created_at} />
                    </div>
                </div>
                <div class="container page">
                    <div class="row article-content">
                        <div class="col-xs-12">
                            { view_body(&article.body) }
                            <ul class="tag-list">
                                {for article.tag_list.iter().map(|tag| {
                                    html! {
                                        <li
                                            class="tag-default tag-pill tag-outline">
                                            { tag }
                                        </li>
                                    }
                                })}
                            </ul>
                        </div>
                    </div>
                    <hr />
                    <div class="article-actions">
                    </div>
                    <div class="row">
                        <CommentList slug={props.slug.clone()} />
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}

/// Dangerously set innerHTML for article body
fn view_body(body: &str) -> Html {
    let parser = pulldown_cmark::Parser::new(body);
    let mut html_text = String::new();
    pulldown_cmark::html::push_html(&mut html_text, parser);

    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    div.set_inner_html(html_text.as_str());
    let node = Node::from(div);
    VNode::VRef(node)
}
