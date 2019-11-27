use yew_router::prelude::*;

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
