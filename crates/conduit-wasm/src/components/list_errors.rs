use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct ListErrors {}

pub enum Msg {}

impl Component for ListErrors {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ListErrors {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                { "ListErrors" }
            </>
        }
    }
}
