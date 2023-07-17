use yew::prelude::*;

use crate::hooks::use_user_context;

#[function_component(Banner)]
pub fn banner() -> Html {
    let user_ctx = use_user_context();
    if user_ctx.is_authenticated() {
        html! {}
    } else {
        html! {
            <div class="banner">
                <div class="container">
                    <h1 class="logo-font">
                        { "conduit" }
                    </h1>
                    <p>{ "A place to share your knowledge." }</p>
                </div>
            </div>
        }
    }
}
