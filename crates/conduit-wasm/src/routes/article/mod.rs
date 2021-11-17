mod article_actions;
mod article_meta;
mod comment;
mod comment_input;
mod comment_list;
mod delete_button;

use web_sys::Node;
use yew::services::fetch::FetchTask;
use yew::virtual_dom::VNode;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::error::Error;
use crate::services::Articles;
use crate::types::{ArticleInfo, ArticleInfoWrapper, UserInfo};

use article_meta::ArticleMeta;
use comment_list::CommentList;

/// Article detail page
pub struct Article {
    articles: Articles,
    article: Option<ArticleInfo>,
    response: Callback<Result<ArticleInfoWrapper, Error>>,
    task: Option<FetchTask>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub slug: String,
    pub current_user: Option<UserInfo>,
}

pub enum Msg {
    Response(Result<ArticleInfoWrapper, Error>),
}

impl Component for Article {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Article {
            articles: Articles::new(),
            article: None,
            response: link.callback(Msg::Response),
            task: None,
            props,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.task = Some(
                self.articles
                    .get(self.props.slug.clone(), self.response.clone()),
            );
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(article_info)) => {
                self.article = Some(article_info.article);
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
        if let Some(article) = &self.article {
            let can_modify = if let Some(user_info) = &self.props.current_user {
                user_info.username == article.author.username
            } else {
                false
            };
            let created_at = article.created_at.format("%B %e, %Y").to_string();

            html! {
                <div class="article-page">
                    <div class="banner">
                        <div class="container">
                            <h1>{&article.title}</h1>
                            <ArticleMeta
                                slug=article.slug.clone()
                                author=article.author.clone()
                                can_modify=can_modify
                                created_at=created_at />
                        </div>
                    </div>
                    <div class="container page">
                        <div class="row article-content">
                            <div class="col-xs-12">
                                { self.view_body(&article.body) }
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
                            <CommentList slug=self.props.slug.clone() current_user=self.props.current_user.clone() />
                        </div>
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    }
}

impl Article {
    /// Dangerously set innerHTML for article body
    fn view_body(&self, body: &str) -> Html {
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
}
