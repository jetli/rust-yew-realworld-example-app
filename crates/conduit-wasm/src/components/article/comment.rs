use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

use super::delete_button::DeleteButton;
use crate::types::{CommentInfo, UserInfo};

pub struct Comment {
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub slug: String,
    #[props(required)]
    pub comment: CommentInfo,
    #[props(required)]
    pub current_user: Option<UserInfo>,
}

pub enum Msg {}

impl Component for Comment {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Comment { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html<Self> {
        let comment = &self.props.comment;
        let show = if let Some(user_info) = &self.props.current_user {
            user_info.username == comment.author.username
        } else {
            false
        };

        html! {
            <div class="card">
                <div class="card-block">
                    <p class="card-text">{ &comment.body }</p>
                </div>
                <div class="card-footer">
                    <span class="comment-author">
                        <img src={ &comment.author.image } class="comment-author-img" alt={ &comment.author.username } />
                    </span> 
                    { " " }
                    <RouterLink text={ &comment.author.username } link={ format!("#/@{}", &comment.author.username) } classes="comment-author" />
                    <span class="date-posted">
                        { &comment.created_at.format("%B %e, %Y") }
                    </span>
                    { if show {
                        html! { <DeleteButton slug=&self.props.slug comment_id=&comment.id/> }
                    } else {
                        html! { }
                    }}
                </div>
            </div>
        }
    }
}
