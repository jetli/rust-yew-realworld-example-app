use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::components::home::Home;

/// The main app component
pub struct App {}

pub enum Msg {
    DoIt,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {}
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                <Home />
            </>
        }
    }
}
