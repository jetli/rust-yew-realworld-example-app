use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct ArticleActions {}

pub enum Msg {}

impl Component for ArticleActions {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ArticleActions {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                { "ArticleActions" }
            </>
        }
    }
}
