use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Settings {}

pub enum Msg {}

impl Component for Settings {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Settings {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                { "Settings" }
            </>
        }
    }
}
