use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

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

    fn view(&self) -> Html<Self> {
        html! {
            <footer>
                <div class="container">
                    <RouterLink text="conduit" link="#/" classes="logo-font"/>
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
