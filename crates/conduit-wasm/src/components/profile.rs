use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Profile {}

pub enum Msg {}

impl Component for Profile {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Profile {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                { "Profile" }
            </>
        }
    }
}
