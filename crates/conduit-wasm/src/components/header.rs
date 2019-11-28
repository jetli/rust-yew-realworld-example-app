use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

use crate::types::UserInfo;

pub struct Header {
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
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

    fn view(&self) -> Html<Self> {
        html! {
            <nav class="navbar navbar-light">
                <div class="container">
                    <RouterLink text="conduit" link="#/" classes="navbar-brand"/>
                    {
                        if let Some(user_info) = &self.props.current_user {
                            self.logged_in_view(&user_info)
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
    fn logged_out_view(&self) -> Html<Self> {
        html! {
            <ul class="nav navbar-nav pull-xs-right">
                <li class="nav-item">
                    <RouterLink text="Home" link="#/" classes="nav-link"/>
                </li>
                <li class="nav-item">
                    <RouterLink text="Sign in" link="#/login" classes="nav-link"/>
                </li>
                <li class="nav-item">
                    <RouterLink text="Sign up" link="#/register" classes="nav-link"/>
                </li>
            </ul>
        }
    }

    fn logged_in_view(&self, user_info: &UserInfo) -> Html<Self> {
        html! {
            <ul class="nav navbar-nav pull-xs-right">
                <li class="nav-item">
                    <RouterLink text="Home" link="#/" classes="nav-link"/>
                </li>
                <li class="nav-item">
                    <RouterLink text="New Post" link="#/editor" classes="nav-link"/>
                </li>
                <li class="nav-item">
                    <RouterLink text="Settings" link="#/settings" classes="nav-link"/>
                </li>
                <li class="nav-item">
                    <RouterLink text=&user_info.username link=format!("#/@{}", &user_info.username)  classes="nav-link"/>
                </li>
            </ul>
        }
    }
}
