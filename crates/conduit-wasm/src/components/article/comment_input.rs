use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct CommentInput {}

pub enum Msg {}

impl Component for CommentInput {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        CommentInput {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                { "CommentInput" }
            </>
        }
    }
}
