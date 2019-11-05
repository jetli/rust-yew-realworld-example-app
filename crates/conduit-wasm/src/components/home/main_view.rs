use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct MainView {}

pub enum Msg {}

impl Component for MainView {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MainView {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>{ "MainView" }</div>
        }
    }
}
