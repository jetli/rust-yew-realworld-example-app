mod article_actions;
mod article_meta;
mod comment;
mod comment_container;
mod comment_input;
mod comment_list;
mod delete_button;

use pulldown_cmark;
use stdweb::js;
use stdweb::unstable::TryFrom;
use stdweb::web::Node;
use yew::services::fetch::FetchTask;
use yew::virtual_dom::VNode;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::agent::Articles;
use crate::error::Error;
use crate::types::{ArticleInfo, ArticleInfoWrapper, UserInfo};

use article_meta::ArticleMeta;
use comment_container::CommentContainer;

/// Article detail page
pub struct Article {
    articles: Articles,
    article: Option<ArticleInfo>,
    response: Callback<Result<ArticleInfoWrapper, Error>>,
    task: Option<FetchTask>,
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub slug: String,
    #[props(required)]
    pub current_user: Option<UserInfo>,
}

pub enum Msg {
    Response(Result<ArticleInfoWrapper, Error>),
}

impl Component for Article {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        Article {
            articles: Articles::new(),
            article: None,
            response: link.send_back(Msg::Response),
            task: None,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.task = Some(
            self.articles
                .get(self.props.slug.clone(), self.response.clone()),
        );
        false
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

    fn view(&self) -> Html<Self> {
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
                                slug=&article.slug
                                author=&article.author
                                can_modify=can_modify
                                created_at=created_at />
                        </div>
                    </div>
                    <div class="container page">
                        <div class="row article-content">
                            <div class="col-xs-12">
                                { self.view_body(&article.body) }
                                <ul className="tag-list">
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
                            <CommentContainer slug=&self.props.slug current_user=&self.props.current_user />
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
    fn view_body(&self, body: &String) -> Html<Self> {
        let parser = pulldown_cmark::Parser::new(body);
        let mut html_text = String::new();
        pulldown_cmark::html::push_html(&mut html_text, parser);

        let js_body = js! {
            var div = document.createElement("div");
            div.innerHTML = @{&html_text};
            return div;
        };
        let node = Node::try_from(js_body).expect("convert js");
        let vnode = VNode::VRef(node);
        vnode
    }
}
