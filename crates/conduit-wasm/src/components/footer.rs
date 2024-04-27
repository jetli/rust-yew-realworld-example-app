use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

#[function_component]
pub fn Footer() -> Html {
    html! {
        <footer>
            <div class="container">
                <Link<AppRoute> to={AppRoute::Home} classes="logo-font">{ "conduit" }</Link<AppRoute>>
                <span class="attribution">
                    { "© 2019. An interactive learning project from" }
                    <a href="https://thinkster.io"> { "Thinkster" } </a>
                    { ". Code licensed under MIT." }
                </span>
            </div>
        </footer>
    }
}
