use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct DeleteButton {}

pub enum Msg {}

impl Component for DeleteButton {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        DeleteButton {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                { "DeleteButton" }
            </>
        }
    }
}
