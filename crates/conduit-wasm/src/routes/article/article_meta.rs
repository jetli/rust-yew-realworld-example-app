use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

use super::article_actions::ArticleActions;
use crate::routes::AppRoute;
use crate::types::ProfileInfo;

pub struct ArticleMeta {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[props(required)]
    pub slug: String,
    #[props(required)]
    pub can_modify: bool,
    #[props(required)]
    pub author: ProfileInfo,
    #[props(required)]
    pub created_at: String,
}

pub enum Msg {}

impl Component for ArticleMeta {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ArticleMeta { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="article-meta">
                <img src={ &self.props.author.image } alt={ &self.props.author.username } />

                <div class="info">
                    <RouterAnchor<AppRoute> route=AppRoute::Profile(self.props.author.username.clone()) classes="author" >
                        { &self.props.author.username }
                    </RouterAnchor<AppRoute>>
                    <span class="date">
                        { &self.props.created_at }
                    </span>
                </div>

                <ArticleActions can_modify=self.props.can_modify slug=&self.props.slug />
            </div>
        }
    }
}
