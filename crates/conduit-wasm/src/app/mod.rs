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

use crate::components::{
    footer::Footer, header::Header, user_context_provider::UserContextProvider,
};

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
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
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => html! {<Login />},
        AppRoute::Register => html! {<Register />},
        AppRoute::Home => html! {<Home />},
        AppRoute::Editor { slug } => html! {<Editor slug={Some(slug)}/>},
        AppRoute::EditorCreate => html! {<Editor />},
        AppRoute::Article { slug } => html! {<Article slug={slug} />},
        AppRoute::Settings => html! {<Settings />},
        AppRoute::ProfileFavorites { username } => html! {
            <Profile username={username} tab={ProfileTab::FavoritedBy} />
        },
        AppRoute::Profile { username } => html! {
            <Profile username={username} tab={ProfileTab::ByAuthor} />
        },
        AppRoute::NotFound => html! { "Page not found" },
    }
}

/// The root app component
#[function_component]
pub fn App() -> Html {
    html! {
        <HashRouter>
            <UserContextProvider>
                <Header />
                <Switch<AppRoute> render={switch} />
                <Footer />
            </UserContextProvider>
        </HashRouter>
    }
}
