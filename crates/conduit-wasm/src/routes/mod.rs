//! Routes by yew_router

pub mod article;
pub mod editor;
pub mod home;
pub mod login;
pub mod profile;
pub mod register;
pub mod settings;

use yew::prelude::*;
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
    #[at("/rust-yew-realworld-example-app/login")]
    Login,
    #[at("/rust-yew-realworld-example-app/register")]
    Register,
    #[at("/rust-yew-realworld-example-app/editor/:slug")]
    Editor { slug: String },
    #[at("/rust-yew-realworld-example-app/editor")]
    EditorCreate,
    #[at("/rust-yew-realworld-example-app/article/:slug")]
    Article { slug: String },
    #[at("/rust-yew-realworld-example-app/settings")]
    Settings,
    #[at("/rust-yew-realworld-example-app/:username/favorites")]
    ProfileFavorites { username: String },
    #[at("/rust-yew-realworld-example-app/:username")]
    Profile { username: String },
    #[at("/rust-yew-realworld-example-app/")]
    Home,
    #[not_found]
    #[at("/rust-yew-realworld-example-app/404")]
    NotFound,
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
        AppRoute::NotFound => html! { "Page not found" },
    }
}
