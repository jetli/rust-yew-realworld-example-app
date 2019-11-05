use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Login {}

pub enum Msg {}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Login {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>{ "Login" }</>
        }
    }
}
