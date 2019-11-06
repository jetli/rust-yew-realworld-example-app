use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct ProfileFavorites {}

pub enum Msg {}

impl Component for ProfileFavorites {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ProfileFavorites {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                { "ProfileFavorites" }
            </>
        }
    }
}
