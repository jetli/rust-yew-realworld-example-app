//! The root app contains initial authentication and url routes

use yew::services::fetch::FetchTask;
use yew::{agent::Bridged, html, Bridge, Callback, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

use super::{
    article::Article, editor::Editor, footer::Footer, header::Header, home::Home, login::Login,
    profile::Profile, profile_favorites::ProfileFavorites, register::Register, settings::Settings,
};
use crate::agent::{is_authenticated, Auth};
use crate::error::Error;
use crate::routes::{fix_fragment_router, AppRoute};
use crate::types::{UserInfo, UserInfoWrapper};

/// The root app component
pub struct App {
    auth: Auth,
    current_route: Option<AppRoute>,
    current_user: Option<UserInfo>,
    current_user_response: Callback<Result<UserInfoWrapper, Error>>,
    current_user_task: Option<FetchTask>,
    #[allow(unused)]
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

pub enum Msg {
    CurrentUserResponse(Result<UserInfoWrapper, Error>),
    Route(Route),
    Authenticated(UserInfo),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.send_back(Msg::Route));
        let route_service: RouteService = RouteService::new();
        let route = route_service.get_route();
        let mut route = Route::from(route);
        fix_fragment_router(&mut route);
        App {
            auth: Auth::new(),
            current_route: AppRoute::switch(route),
            router_agent,
            current_user: None,
            current_user_response: link.send_back(Msg::CurrentUserResponse),
            current_user_task: None,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        // Get current user info if a token is available when mounted
        if is_authenticated() {
            let task = self.auth.current(self.current_user_response.clone());
            self.current_user_task = Some(task);
        }
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CurrentUserResponse(Ok(user_info)) => {
                self.current_user = Some(user_info.user);
                self.current_user_task = None;
            }
            Msg::CurrentUserResponse(Err(_)) => {
                self.current_user_task = None;
            }
            Msg::Route(mut route) => {
                fix_fragment_router(&mut route);
                self.current_route = AppRoute::switch(route)
            }
            Msg::Authenticated(user_info) => {
                self.current_user = Some(user_info);
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                <Header current_user=&self.current_user/>
                {
                    // Routes to render sub components
                    if let Some(route) = &self.current_route {
                        match route {
                            AppRoute::Login => html!{<Login callback=Msg::Authenticated/>},
                            AppRoute::Register => html!{<Register callback=Msg::Authenticated />},
                            AppRoute::Home => html!{<Home />},
                            AppRoute::Editor(slug) => html!{<Editor slug=Some(slug.clone())/>},
                            AppRoute::EditorCreate => html!{<Editor />},
                            AppRoute::Article(slug) => html!{<Article slug=slug current_user=&self.current_user />},
                            AppRoute::Settings => html!{<Settings />},
                            AppRoute::ProfileFavorites(username) => html!{<ProfileFavorites />},
                            AppRoute::Profile(username) => html!{<Profile />},
                        }
                    } else {
                        // 404 when route matches no component
                        html! { "No child component available" }
                    }
                }
                <Footer />
            </>
        }
    }
}
