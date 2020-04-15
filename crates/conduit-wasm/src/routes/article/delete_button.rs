use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::error::Error;
use crate::services::Comments;
use crate::types::DeleteWrapper;

/// A component to delete a comment from an article.
pub struct DeleteButton {
    comments: Comments,
    response: Callback<Result<DeleteWrapper, Error>>,
    task: Option<FetchTask>,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub slug: String,
    pub comment_id: u32,
    pub callback: Callback<u32>,
}

pub enum Msg {
    DeleteComment,
    Response(Result<DeleteWrapper, Error>),
}

impl Component for DeleteButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        DeleteButton {
            comments: Comments::new(),
            response: link.callback(Msg::Response),
            task: None,
            props,
            link,
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
            Msg::Response(Ok(_)) => {
                self.props.callback.emit(self.props.comment_id);
            }
            Msg::Response(Err(_)) => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::DeleteComment);
        html! {
            <span class="mod-options">
                <i class="ion-trash-a" onclick=onclick ></i>
            </span>
        }
    }
}
