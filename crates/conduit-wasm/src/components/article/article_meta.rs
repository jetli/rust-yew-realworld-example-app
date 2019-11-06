use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct ArticleMeta {}

pub enum Msg {}

impl Component for ArticleMeta {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ArticleMeta {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                { "ArticleMeta" }
            </>
        }
    }
}
