use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

use crate::components::{header::Header, home::Home, login::Login, register::Register};

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
                <Header />
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Login => html!{<Login />},
                            AppRoute::Register => html!{<Register />},
                            AppRoute::Home => html!{<Home />},
                        }
                    })
                />
            </>
        }
    }
}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/#login"]
    Login,
    #[to = "/#register"]
    Register,
    #[to = "/"]
    Home,
}
