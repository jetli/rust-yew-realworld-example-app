use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

use super::{
    article::Article, editor::Editor, header::Header, home::Home, login::Login, profile::Profile,
    profile_favorites::ProfileFavorites, register::Register, settings::Settings,
};
use crate::types::UserInfo;

/// The main app component
pub struct App {
    current_user: Option<UserInfo>,
}

pub enum Msg {
    LoginReady(UserInfo),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            current_user: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoginReady(user_info) => {
                self.current_user = Some(user_info);
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                <Header />
                <Router<AppRoute, Msg>
                    callback = From::from
                    render = Router::render(|switch: AppRoute| -> Html<Router<AppRoute, Msg>>{
                        match switch {
                            AppRoute::Login => html!{<Login />},
                            AppRoute::Register => html!{<Register />},
                            AppRoute::Home => html!{<Home />},
                            AppRoute::Editor(slug) => html!{<Editor />},
                            AppRoute::EditorCreate => html!{<Editor />},
                            AppRoute::Article(id) => html!{<Article />},
                            AppRoute::Settings => html!{<Settings />},
                            AppRoute::ProfileFavorites(username) => html!{<ProfileFavorites />},
                            AppRoute::Profile(username) => html!{<Profile />},
                        }
                    })
                />
            </>
        }
    }
}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/#/login"]
    Login,
    #[to = "/#/register"]
    Register,
    #[to = "/#/editor/{slug}"]
    Editor(String),
    #[to = "/#/editor"]
    EditorCreate,
    #[to = "/#/article/{id}"]
    Article(String),
    #[to = "/#/settings"]
    Settings,
    #[to = "/#/@{username}/favorites"]
    ProfileFavorites(String),
    #[to = "/#/@{username}"]
    Profile(String),
    #[to = "/"]
    Home,
}
