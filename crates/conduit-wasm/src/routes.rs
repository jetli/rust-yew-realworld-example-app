use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "#/login"]
    Login,
    #[to = "#/register"]
    Register,
    #[to = "#/editor/{slug}"]
    Editor(String),
    #[to = "#/editor"]
    EditorCreate,
    #[to = "#/article/{id}"]
    Article(String),
    #[to = "#/settings"]
    Settings,
    #[to = "#/@{username}/favorites"]
    ProfileFavorites(String),
    #[to = "#/@{username}"]
    Profile(String),
    #[to = "#/"]
    Home,
}

pub fn fix_fragment_router(route: &mut Route) {
    let r = route.route.as_str();
    if r.eq("/") {
        route.route = "#/".to_string();
    } else {
        if r.starts_with("/#") {
            route.route = r[1..].to_string();
        }
    }
}
