use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::types::UserInfo;

pub struct CommentInput {
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub slug: String,
    #[props(required)]
    pub current_user: UserInfo,
}

pub enum Msg {}

impl Component for CommentInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        CommentInput { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <form class="card comment-form">
                <div class="card-block">
                    <textarea class="form-control"
                        placeholder="Write a comment..."
                        rows="3">
                    </textarea>
                </div>
                <div class="card-footer">
                    {if let Some(image) = &self.props.current_user.image {
                        html! {
                            <img
                                src={ image }
                                class="comment-author-img"
                                alt={ &self.props.current_user.username} />
                        }
                    } else {
                        html! { }
                    }}
                    <button
                        class="btn btn-sm btn-primary"
                        type="submit">
                        { "Post Comment" }
                    </button>
                </div>
            </form>
        }
    }
}
