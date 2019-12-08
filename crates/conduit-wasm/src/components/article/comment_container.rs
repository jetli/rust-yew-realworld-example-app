use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

use super::comment_input::CommentInput;
use super::comment_list::CommentList;
use crate::types::UserInfo;

pub struct CommentContainer {
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub slug: String,
    #[props(required)]
    pub current_user: Option<UserInfo>,
}

pub enum Msg {}

impl Component for CommentContainer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        CommentContainer { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html<Self> {
        if let Some(user_info) = &self.props.current_user {
            html! {
                <div class="col-xs-12 col-md-8 offset-md-2">
                    <div>
                        <CommentInput slug=&self.props.slug current_user=user_info />
                    </div>

                    <CommentList slug=&self.props.slug current_user=&self.props.current_user />
                </div>
            }
        } else {
            html! {
                <div class="col-xs-12 col-md-8 offset-md-2">
                    <p>
                        <RouterLink text="Sign in" link="#/login"/>
                        { " or " }
                        <RouterLink text="sign up" link="#/register"/   >
                        { " to add comments on this article." }
                    </p>

                    <CommentList slug=&self.props.slug current_user=&self.props.current_user />
                </div>
            }
        }
    }
}
