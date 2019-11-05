use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Register {}

pub enum Msg {}

impl Component for Register {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Register {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>{ "Register" }</>
        }
    }
}
