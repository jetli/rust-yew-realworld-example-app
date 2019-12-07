use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

use super::article_preview::ArticlePreview;
use super::list_pagination::ListPagination;
use crate::agent::Articles;
use crate::error::Error;
use crate::types::ArticleListInfo;

/// List of articles component
pub struct ArticleList {
    articles: Articles,
    article_list: Option<ArticleListInfo>,
    response: Callback<Result<ArticleListInfo, Error>>,
    task: Option<FetchTask>,
    current_page: u32,
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub filter: ArticleListFilter,
}

pub enum Msg {
    Response(Result<ArticleListInfo, Error>),
    PaginationChanged(u32),
}

#[derive(Clone, Debug)]
pub enum ArticleListFilter {
    All,
    ByAuthor(String),
    ByTag(String),
    FavoritedBy(String),
    Feed,
}

impl Component for ArticleList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        ArticleList {
            articles: Articles::new(),
            article_list: None,
            response: link.send_back(Msg::Response),
            task: None,
            current_page: 0,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.request();
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(article_list)) => {
                self.article_list = Some(article_list);
                self.task = None;
            }
            Msg::Response(Err(_)) => {
                self.task = None;
            }
            Msg::PaginationChanged(current_page) => {
                self.current_page = current_page;
                self.request();
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        self.current_page = 0;
        self.request();
        false
    }

    fn view(&self) -> Html<Self> {
        if let Some(article_list) = &self.article_list {
            if !article_list.articles.is_empty() {
                html! {
                    <>
                        {for article_list.articles.iter().map(|article| {
                            html! { <ArticlePreview article=article /> }
                        })}
                        <ListPagination
                            articles_count=article_list.articles_count
                            current_page=self.current_page
                            callback=Msg::PaginationChanged />
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
    fn request(&mut self) {
        match self.props.filter.clone() {
            ArticleListFilter::All => {
                self.task = Some(self.articles.all(self.current_page, self.response.clone()));
            }
            ArticleListFilter::ByAuthor(author) => {
                self.task = Some(self.articles.by_author(
                    author,
                    self.current_page,
                    self.response.clone(),
                ));
            }
            ArticleListFilter::ByTag(tag) => {
                self.task = Some(self.articles.by_tag(
                    tag,
                    self.current_page,
                    self.response.clone(),
                ));
            }
            ArticleListFilter::FavoritedBy(author) => {
                self.task = Some(self.articles.favorited_by(
                    author,
                    self.current_page,
                    self.response.clone(),
                ));
            }
            ArticleListFilter::Feed => {
                self.task = Some(self.articles.feed(self.response.clone()));
            }
        }
    }
}
