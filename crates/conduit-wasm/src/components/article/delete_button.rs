use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::agent::Comments;
use crate::error::Error;
use crate::types::DeleteWrapper;

pub struct DeleteButton {
    comments: Comments,
    response: Callback<Result<DeleteWrapper, Error>>,
    task: Option<FetchTask>,
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub slug: String,
    #[props(required)]
    pub comment_id: u32,
}

pub enum Msg {
    DeleteComment,
    Response(Result<DeleteWrapper, Error>),
}

impl Component for DeleteButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        DeleteButton {
            comments: Comments::new(),
            response: link.send_back(Msg::Response),
            task: None,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DeleteComment => {
                self.task = Some(self.comments.delete(
                    self.props.slug.clone(),
                    self.props.comment_id,
                    self.response.clone(),
                ));
            }
            Msg::Response(Ok(_)) => {}
            Msg::Response(Err(_)) => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <span class="mod-options">
                <i class="ion-trash-a" onclick=|_| Msg::DeleteComment ></i>
            </span>
        }
    }
}
