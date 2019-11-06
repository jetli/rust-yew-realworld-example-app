use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

pub struct Header {}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <nav class="navbar navbar-light">
                <div class="container">
                    <RouterLink text="conduit" link="/" classes="navbar-brand"/>
                    <ul class="nav navbar-nav pull-xs-right">
                        <li class="nav-item">
                            <RouterLink text="Home" link="/" classes="nav-link"/>
                        </li>
                        <li class="nav-item">
                            <RouterLink text="Sign in" link="/login" classes="nav-link"/>
                        </li>
                        <li class="nav-item">
                            <RouterLink text="Sign up" link="/register" classes="nav-link"/>
                        </li>
                    </ul>
                </div>
            </nav>
        }
    }
}
