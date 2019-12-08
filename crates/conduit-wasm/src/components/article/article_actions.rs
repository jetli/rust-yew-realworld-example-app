use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

pub struct ArticleActions {
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub slug: String,
    #[props(required)]
    pub can_modify: bool,
}

pub enum Msg {}

impl Component for ArticleActions {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ArticleActions { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html<Self> {
        if self.props.can_modify {
            html! {
                <span>
                    <RouterLink text={ "Edit Article" } link={ format!("#/editor/{}", &self.props.slug) } classes="btn btn-outline-secondary btn-sm" />
                    { " " }
                    <button class="btn btn-outline-danger btn-sm" >
                        <i class="ion-trash-a"></i> { "Delete Article" }
                    </button>
                </span>
            }
        } else {
            html! {
                <span>
                </span>
            }
        }
    }
}
