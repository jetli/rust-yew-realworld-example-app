use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {}

pub enum Msg {
    DoIt,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => true,
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <button onclick=|_| Msg::DoIt>{ "Click me!" }</button>
        }
    }
}
