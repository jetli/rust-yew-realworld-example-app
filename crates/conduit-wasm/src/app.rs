//! The root app contains initial authentication and url routes

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    footer::Footer, header::Header, user_context_provider::UserContextProvider,
};
use crate::routes::{switch, AppRoute};

/// The root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <UserContextProvider>
            <HashRouter>
                <Header />
                <Switch<AppRoute> render={switch} />
                <Footer />
            </HashRouter>
        </UserContextProvider>
    }
}
