use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Comment {}

pub enum Msg {}

impl Component for Comment {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Comment {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                { "Comment" }
            </>
        }
    }
}
