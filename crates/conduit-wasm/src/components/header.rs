use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::types::UserInfo;

pub struct Header {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current_user: Option<UserInfo>,
}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header { props }
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
            <nav class="navbar navbar-light">
                <div class="container">
                    <RouterAnchor<AppRoute> route=AppRoute::Home classes="navbar-brand">
                        { "conduit" }
                    </RouterAnchor<AppRoute>>
                    {
                        if let Some(user_info) = &self.props.current_user {
                            self.logged_in_view(user_info)
                        } else {
                            self.logged_out_view()
                        }
                    }
                </div>
            </nav>
        }
    }
}

impl Header {
    fn logged_out_view(&self) -> Html {
        html! {
            <ul class="nav navbar-nav pull-xs-right">
                <li class="nav-item">
                    <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                        { "Home" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li class="nav-item">
                    <RouterAnchor<AppRoute> route=AppRoute::Login classes="nav-link">
                        { "Sign in" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li class="nav-item">
                    <RouterAnchor<AppRoute> route=AppRoute::Register classes="nav-link">
                        { "Sign up" }
                    </RouterAnchor<AppRoute>>
                </li>
            </ul>
        }
    }

    fn logged_in_view(&self, user_info: &UserInfo) -> Html {
        html! {
            <ul class="nav navbar-nav pull-xs-right">
                <li class="nav-item">
                    <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                        { "Home" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li class="nav-item">
                    <RouterAnchor<AppRoute> route=AppRoute::EditorCreate classes="nav-link">
                        { "New Post" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li class="nav-item">
                    <RouterAnchor<AppRoute> route=AppRoute::Settings classes="nav-link">
                        { "Settings" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li class="nav-item">
                    <RouterAnchor<AppRoute> route=AppRoute::Profile(user_info.username.clone())  classes="nav-link">
                        { &user_info.username }
                    </RouterAnchor<AppRoute>>
                </li>
            </ul>
        }
    }
}
