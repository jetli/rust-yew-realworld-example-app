mod article_actions;
mod article_meta;
mod comment;
mod comment_container;
mod comment_input;
mod comment_list;
mod delete_button;

use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Article {}

pub enum Msg {}

impl Component for Article {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Article {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                { "Article" }
            </>
        }
    }
}
