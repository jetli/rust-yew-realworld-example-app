use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct CommentList {}

pub enum Msg {}

impl Component for CommentList {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        CommentList {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                { "CommentList" }
            </>
        }
    }
}
