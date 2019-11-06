use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct ListPagination {}

pub enum Msg {}

impl Component for ListPagination {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ListPagination {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                { "ListPagination" }
            </>
        }
    }
}
