use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct CommentContainer {}

pub enum Msg {}

impl Component for CommentContainer {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        CommentContainer {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                { "CommentContainer" }
            </>
        }
    }
}
