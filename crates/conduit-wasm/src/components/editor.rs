use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Editor {}

pub enum Msg {}

impl Component for Editor {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Editor {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                { "Editor" }
            </>
        }
    }
}
