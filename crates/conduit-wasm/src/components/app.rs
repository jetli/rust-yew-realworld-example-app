use yew::{agent::Bridged, html, Bridge, Component, ComponentLink, Html, ShouldRender};
use yew_router::{agent::RouteAgent, prelude::*, route::Route, service::RouteService};

use super::{
    article::Article, editor::Editor, header::Header, home::Home, login::Login, profile::Profile,
    profile_favorites::ProfileFavorites, register::Register, settings::Settings,
};
use crate::routes::AppRoute;
use crate::types::UserInfo;

/// The main app component
pub struct App {
    current_route: Option<AppRoute>,
    #[allow(unused)]
    router_agent: Box<dyn Bridge<RouteAgent<()>>>,
    current_user: Option<UserInfo>,
}

pub enum Msg {
    Route(Route<()>),
    LoginReady(UserInfo),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.send_back(Msg::Route));
        let route_service: RouteService<()> = RouteService::new();
        let route = route_service.get_route();
        let route = Route::<()>::from(route);
        App {
            current_route: AppRoute::switch(route),
            router_agent,
            current_user: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Route(route) => self.current_route = AppRoute::switch(route),
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
                {
                    if let Some(route) = &self.current_route {
                        match route {
                            AppRoute::Login => html!{<Login callback=Msg::LoginReady/>},
                            AppRoute::Register => html!{<Register />},
                            AppRoute::Home => html!{<Home />},
                            AppRoute::Editor(slug) => html!{<Editor />},
                            AppRoute::EditorCreate => html!{<Editor />},
                            AppRoute::Article(id) => html!{<Article />},
                            AppRoute::Settings => html!{<Settings />},
                            AppRoute::ProfileFavorites(username) => html!{<ProfileFavorites />},
                            AppRoute::Profile(username) => html!{<Profile />},
                        }
                    } else {
                        html! { "No child component available" }
                    }
                }
            </>
        }
    }
}
