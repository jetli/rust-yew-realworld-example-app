use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

use super::comment::Comment;
use super::comment_input::CommentInput;
use crate::error::Error;
use crate::routes::AppRoute;
use crate::services::Comments;
use crate::types::{CommentInfo, CommentListInfo, UserInfo};

/// A comment list component of an article.
pub struct CommentList {
    comments: Comments,
    comment_list: Option<Vec<CommentInfo>>,
    response: Callback<Result<CommentListInfo, Error>>,
    task: Option<FetchTask>,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub slug: String,
    pub current_user: Option<UserInfo>,
}

pub enum Msg {
    Response(Result<CommentListInfo, Error>),
    CommentAdded(CommentInfo),
    CommentDeleted(u32),
}

impl Component for CommentList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CommentList {
            comments: Comments::new(),
            comment_list: None,
            response: link.callback(Msg::Response),
            task: None,
            props,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.task = Some(
                self.comments
                    .for_article(self.props.slug.clone(), self.response.clone()),
            );
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(comment_list)) => {
                self.comment_list = Some(comment_list.comments);
                self.task = None;
            }
            Msg::Response(Err(_)) => {
                self.task = None;
            }
            Msg::CommentAdded(comment_info) => {
                if let Some(comment_list) = &mut self.comment_list {
                    comment_list.insert(0, comment_info);
                }
            }
            Msg::CommentDeleted(comment_id) => {
                if let Some(comment_list) = &mut self.comment_list {
                    comment_list.retain(|c| c.id != comment_id);
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        if let Some(comment_list) = &self.comment_list {
            html! {
                <div class="col-xs-12 col-md-8 offset-md-2">
                    {
                        if let Some(user_info) = &self.props.current_user {
                            let callback = self.link.callback(Msg::CommentAdded);
                            html! {
                                <div>
                                    <CommentInput
                                        slug=self.props.slug.clone()
                                        current_user=user_info.clone()
                                        callback=callback />
                                </div>
                            }
                        } else {
                            html! {
                                <p>
                                    <RouterAnchor<AppRoute> route=AppRoute::Login classes="nav-link">
                                        { "Sign in" }
                                    </RouterAnchor<AppRoute>>
                                    { " or " }
                                    <RouterAnchor<AppRoute> route=AppRoute::Register classes="nav-link">
                                        { "sign up" }
                                    </RouterAnchor<AppRoute>>
                                    { " to add comments on this article." }
                                </p>
                            }
                        }
                    }
                    <div>
                        {for comment_list.iter().map(|comment| {
                            let callback = self.link.callback(Msg::CommentDeleted);
                            html! {
                                <Comment
                                    slug=self.props.slug.clone()
                                    comment=comment.clone()
                                    current_user=self.props.current_user.clone()
                                    callback=callback />
                            }
                        })}
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    }
}
