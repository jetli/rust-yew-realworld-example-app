use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;

use yew::prelude::*;

use crate::components::list_errors::ListErrors;
use crate::hooks::use_user_context;
use crate::services::auth::*;
use crate::types::{UserUpdateInfo, UserUpdateInfoWrapper};

/// Update settings of the author or logout
#[function_component(Settings)]
pub fn settings() -> Html {
    let user_ctx = use_user_context();
    let error = use_state(|| None);
    let update_info = use_state(UserUpdateInfo::default);
    let password = use_state(String::default);

    {
        let update_info = update_info.clone();
        let error = error.clone();
        let user_ctx = user_ctx.clone();
        use_effect_with_deps(
            move |_| {
                let user_ctx = user_ctx.clone();
                spawn_local(async move {
                    if user_ctx.is_authenticated() {
                        let user_info = current().await;
                        match user_info {
                            Ok(user_info) => {
                                update_info.set(UserUpdateInfo {
                                    email: user_info.user.email,
                                    username: user_info.user.username,
                                    password: None,
                                    image: user_info.user.image.unwrap_or_default(),
                                    bio: user_info.user.bio.unwrap_or_default(),
                                });
                                error.set(None);
                            }
                            Err(e) => error.set(Some(e)),
                        }
                    }
                });

                || ()
            },
            (),
        );
    }

    let onsubmit = {
        let error = error.clone();
        let update_info = update_info.clone();
        let password = password.clone();
        let user_ctx = user_ctx.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default(); /* Prevent event propagation */
            let mut request = UserUpdateInfoWrapper {
                user: (*update_info).clone(),
            };
            if (*password).is_empty() {
                request.user.password = Some((*password).clone());
            }
            let user_ctx = user_ctx.clone();
            let error = error.clone();
            spawn_local(async move {
                let user_info = save(request).await;
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
    let oninput_image = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.image = input.value();
            update_info.set(info);
        })
    };
    let oninput_username = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.username = input.value();
            update_info.set(info);
        })
    };
    let oninput_bio = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.bio = input.value();
            update_info.set(info);
        })
    };
    let oninput_email = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.email = input.value();
            update_info.set(info);
        })
    };
    let oninput_password = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };
    let onclick = {
        Callback::from(move |_| {
            // Logout current user
            user_ctx.logout();
        })
    };

    html! {
        <div class="settings-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">{ "Your Settings" }</h1>
                        <ListErrors error={(*error).clone()}/>
                        <form {onsubmit}>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control"
                                        type="text"
                                        placeholder="URL of profile picture"
                                        value={update_info.image.clone()}
                                        oninput={oninput_image} />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="text"
                                        placeholder="Username"
                                        value={update_info.username.clone()}
                                        oninput={oninput_username} />
                                </fieldset>
                                <fieldset class="form-group">
                                    <textarea
                                        class="form-control form-control-lg"
                                        rows="8"
                                        placeholder="Short bio about you"
                                        value={update_info.bio.clone()}
                                        oninput={oninput_bio} >
                                    </textarea>
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="email"
                                        placeholder="Email"
                                        value={update_info.email.clone()}
                                        oninput={oninput_email} />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="New Password"
                                        value={(*password).clone()}
                                        oninput={oninput_password} />
                                </fieldset>
                                <button
                                    class="btn btn-lg btn-primary pull-xs-right"
                                    type="submit"
                                    disabled=false>
                                    { "Update Settings" }
                                </button>
                            </fieldset>
                        </form>
                        <hr />
                        <button
                            class="btn btn-outline-danger"
                            {onclick} >
                            { "Or click here to logout."}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
