use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::components::list_errors::ListErrors;
use crate::hooks::use_user_context;
use crate::routes::AppRoute;
use crate::services::auth::*;
use crate::types::{RegisterInfo, RegisterInfoWrapper};

/// Register page
#[function_component(Register)]
pub fn register() -> Html {
    let user_ctx = use_user_context();
    let register_info = use_state(RegisterInfo::default);
    let user_register = {
        let register_info = register_info.clone();
        use_async(async move {
            let request = RegisterInfoWrapper {
                user: (*register_info).clone(),
            };
            register(request).await
        })
    };

    {
        use_effect_with_deps(
            move |user_register| {
                if let Some(user_info) = &user_register.data {
                    user_ctx.login(user_info.user.clone());
                }
                || ()
            },
            user_register.clone(),
        );
    }

    let onsubmit = {
        let user_register = user_register.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default(); /* Prevent event propagation */
            user_register.run();
        })
    };
    let oninput_username = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.username = input.value();
            register_info.set(info);
        })
    };
    let oninput_email = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.email = input.value();
            register_info.set(info);
        })
    };
    let oninput_password = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.password = input.value();
            register_info.set(info);
        })
    };

    html! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">{ "Sign Up" }</h1>
                        <p class="text-xs-center">
                            <Link<AppRoute> to={AppRoute::Login}>
                                { "Have an account?" }
                            </Link<AppRoute>>
                        </p>
                        <ListErrors error={user_register.error.clone()} />
                        <form {onsubmit}>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="text"
                                        placeholder="Username"
                                        value={register_info.username.clone()}
                                        oninput={oninput_username}
                                        />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="email"
                                        placeholder="Email"
                                        value={register_info.email.clone()}
                                        oninput={oninput_email}
                                        />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="Password"
                                        value={register_info.password.clone()}
                                        oninput={oninput_password}
                                        />
                                </fieldset>
                                <button
                                    class="btn btn-lg btn-primary pull-xs-right"
                                    type="submit"
                                    disabled=false>
                                    { "Sign up" }
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
