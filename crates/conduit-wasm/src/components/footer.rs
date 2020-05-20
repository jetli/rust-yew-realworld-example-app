use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

use crate::routes::AppRoute;

pub struct Footer {}

pub enum Msg {}

impl Component for Footer {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Footer {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <footer>
                <div class="container">
                    <RouterAnchor<AppRoute> route=AppRoute::Home classes="logo-font">{ "conduit" }</RouterAnchor<AppRoute>>
                    <span class="attribution">
                        { "© 2019. An interactive learning project from" }
                        <a href="https://thinkster.io"> { "Thinkster" } </a>
                        { ". Code licensed under MIT.aaa" }
                    </span>
                </div>
            </footer>
        }
    }
}
