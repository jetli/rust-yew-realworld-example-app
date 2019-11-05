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
            <div>
                <RouterLink text="Login" link="/login"/> { "|" } <RouterLink text="Register" link="/register"/>
            </div>
        }
    }
}
