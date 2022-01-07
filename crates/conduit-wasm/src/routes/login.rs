use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::list_errors::ListErrors;
use crate::hooks::use_user_context;
use crate::routes::AppRoute;
use crate::services::auth::*;
use crate::types::{LoginInfo, LoginInfoWrapper};

/// Login page
#[function_component(Login)]
pub fn login() -> Html {
    let user_ctx = use_user_context();
    let error = use_state(|| None);
    let login_info = use_state(LoginInfo::default);

    let onsubmit = {
        let error = error.clone();
        let login_info = login_info.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default(); /* Prevent event propagation */
            let request = LoginInfoWrapper {
                user: (*login_info).clone(),
            };
            let user_ctx = user_ctx.clone();
            let error = error.clone();
            spawn_local(async move {
                let user_info = login(request).await;
                match user_info {
                    Ok(user_info) => {
                        user_ctx.login(user_info.user);
                        error.set(None);
                    }
                    Err(e) => error.set(Some(e)),
                }
            });
        })
    };
    let oninput_email = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.email = input.value();
            login_info.set(info);
        })
    };
    let oninput_password = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.password = input.value();
            login_info.set(info);
        })
    };

    html! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">{ "Sign In" }</h1>
                        <p class="text-xs-center">
                            <Link<AppRoute> to={AppRoute::Register}>
                                { "Need an account?" }
                            </Link<AppRoute>>
                        </p>
                        <ListErrors error={(*error).clone()} />
                        <form {onsubmit}>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="email"
                                        placeholder="Email"
                                        value={login_info.email.clone()}
                                        oninput={oninput_email}
                                        />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="Password"
                                        value={login_info.password.clone()}
                                        oninput={oninput_password}
                                        />
                                </fieldset>
                                <button
                                    class="btn btn-lg btn-primary pull-xs-right"
                                    type="submit"
                                    disabled=false>
                                    { "Sign in" }
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
