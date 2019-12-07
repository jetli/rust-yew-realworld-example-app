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

use crate::agent::{Articles, Comments};
use crate::error::Error;
use crate::types::{ArticleInfo, ArticleInfoWrapper, CommentInfo, CommentListInfo};

use article_meta::ArticleMeta;
use comment_container::CommentContainer;

/// Article detail page
pub struct Article {
    articles: Articles,
    article: Option<ArticleInfo>,
    article_response: Callback<Result<ArticleInfoWrapper, Error>>,
    article_task: Option<FetchTask>,
    comments: Comments,
    comment_list: Option<Vec<CommentInfo>>,
    comment_response: Callback<Result<CommentListInfo, Error>>,
    comment_task: Option<FetchTask>,
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub slug: String,
}

pub enum Msg {
    ArticleResponse(Result<ArticleInfoWrapper, Error>),
    CommentResponse(Result<CommentListInfo, Error>),
}

impl Component for Article {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        Article {
            articles: Articles::new(),
            article: None,
            article_response: link.send_back(Msg::ArticleResponse),
            article_task: None,
            comments: Comments::new(),
            comment_list: None,
            comment_response: link.send_back(Msg::CommentResponse),
            comment_task: None,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.article_task = Some(
            self.articles
                .get(self.props.slug.clone(), self.article_response.clone()),
        );
        self.comment_task = Some(
            self.comments
                .for_article(self.props.slug.clone(), self.comment_response.clone()),
        );
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ArticleResponse(Ok(article_info)) => {
                self.article = Some(article_info.article);
                self.article_task = None;
            }
            Msg::ArticleResponse(Err(_)) => {
                self.article_task = None;
            }
            Msg::CommentResponse(Ok(comment_list)) => {
                self.comment_list = Some(comment_list.comments);
                self.comment_task = None;
            }
            Msg::CommentResponse(Err(_)) => {
                self.comment_task = None;
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        if let Some(article) = &self.article {
            html! {
                <div class="article-page">
                    <div class="banner">
                        <div class="container">
                            <h1>{&article.title}</h1>
                            <ArticleMeta />
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
                            <CommentContainer />
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
