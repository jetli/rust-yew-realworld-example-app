//! Routes by yew_router

pub mod article;
pub mod editor;
pub mod home;
pub mod login;
pub mod profile;
pub mod register;
pub mod settings;

use yew::{html, Html};
use yew_router::prelude::*;

use article::Article;
use editor::Editor;
use home::Home;
use login::Login;
use profile::{Profile, ProfileTab};
use register::Register;
use settings::Settings;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/editor/:slug")]
    Editor { slug: String },
    #[at("/editor")]
    EditorCreate,
    #[at("/article/:slug")]
    Article { slug: String },
    #[at("/settings")]
    Settings,
    #[at("/:username/favorites")]
    ProfileFavorites { username: String },
    #[at("/:username")]
    Profile { username: String },
    #[at("/")]
    Home,
}

pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Login => html! {<Login />},
        AppRoute::Register => html! {<Register />},
        AppRoute::Home => html! {<Home />},
        AppRoute::Editor { slug } => html! {<Editor slug={Some(slug.clone())}/>},
        AppRoute::EditorCreate => html! {<Editor />},
        AppRoute::Article { slug } => html! {<Article slug={slug.clone()} />},
        AppRoute::Settings => html! {<Settings />},
        AppRoute::ProfileFavorites { username } => html! {
            <Profile username={username.clone()} tab={ProfileTab::FavoritedBy} />
        },
        AppRoute::Profile { username } => html! {
            <Profile username={username.clone()} tab={ProfileTab::ByAuthor} />
        },
    }
}
