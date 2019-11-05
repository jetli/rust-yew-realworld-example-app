use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Tags {}

pub enum Msg {}

impl Component for Tags {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Tags {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>{ "Tags" }</div>
        }
    }
}
