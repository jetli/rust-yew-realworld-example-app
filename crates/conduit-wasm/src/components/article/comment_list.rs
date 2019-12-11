use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

use super::comment::Comment;
use super::comment_input::CommentInput;
use crate::agent::Comments;
use crate::error::Error;
use crate::types::{CommentInfo, CommentListInfo, UserInfo};

pub struct CommentList {
    comments: Comments,
    comment_list: Option<Vec<CommentInfo>>,
    response: Callback<Result<CommentListInfo, Error>>,
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
    Response(Result<CommentListInfo, Error>),
}

impl Component for CommentList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        CommentList {
            comments: Comments::new(),
            comment_list: None,
            response: link.send_back(Msg::Response),
            task: None,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.task = Some(
            self.comments
                .for_article(self.props.slug.clone(), self.response.clone()),
        );
        false
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
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html<Self> {
        if let Some(comment_list) = &self.comment_list {
            html! {
                <div class="col-xs-12 col-md-8 offset-md-2">
                    {
                        if let Some(user_info) = &self.props.current_user {
                            html! {
                                <div>
                                    <CommentInput slug=&self.props.slug current_user=user_info />
                                </div>
                            }
                        } else {
                            html! {
                                <p>
                                    <RouterLink text="Sign in" link="#/login"/>
                                    { " or " }
                                    <RouterLink text="sign up" link="#/register"/   >
                                    { " to add comments on this article." }
                                </p>
                            }
                        }
                    }
                    <div>
                        {for comment_list.iter().map(|comment| {
                            html! {
                                <Comment slug=&self.props.slug comment=comment current_user=&self.props.current_user />
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
