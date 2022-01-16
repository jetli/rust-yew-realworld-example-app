use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::{use_async, use_mount};

use crate::components::list_errors::ListErrors;
use crate::hooks::use_user_context;
use crate::services::auth::*;
use crate::types::{UserUpdateInfo, UserUpdateInfoWrapper};

/// Update settings of the author or logout
#[function_component(Settings)]
pub fn settings() -> Html {
    let user_ctx = use_user_context();
    let update_info = use_state(UserUpdateInfo::default);
    let password = use_state(String::default);
    let user_info = use_async(async move { current().await });
    let user_update = {
        let update_info = update_info.clone();
        let password = password.clone();
        use_async(async move {
            let mut request = UserUpdateInfoWrapper {
                user: (*update_info).clone(),
            };
            if !(*password).is_empty() {
                request.user.password = Some((*password).clone());
            }
            save(request).await
        })
    };

    {
        let user_ctx = user_ctx.clone();
        let user_info = user_info.clone();
        use_mount(move || {
            if user_ctx.is_authenticated() {
                user_info.run();
            }
        });
    }

    {
        let user_info = user_info.clone();
        let update_info = update_info.clone();
        use_effect_with_deps(
            move |user_info| {
                if let Some(user_info) = &user_info.data {
                    update_info.set(UserUpdateInfo {
                        email: user_info.user.email.clone(),
                        username: user_info.user.username.clone(),
                        password: None,
                        image: user_info.user.image.clone().unwrap_or_default(),
                        bio: user_info.user.bio.clone().unwrap_or_default(),
                    });
                }
                || ()
            },
            user_info,
        );
    }

    {
        let user_ctx = user_ctx.clone();
        let user_update = user_update.clone();
        use_effect_with_deps(
            move |user_update| {
                if let Some(user_info) = &user_update.data {
                    // Login current user again to update user info.
                    user_ctx.login(user_info.user.clone());
                }
                || ()
            },
            user_update,
        );
    }

    let onsubmit = {
        let user_update = user_update.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default(); /* Prevent event propagation */
            let user_update = user_update.clone();
            user_update.run();
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
                        <ListErrors error={user_info.error.clone()}/>
                        <ListErrors error={user_update.error.clone()}/>
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
                                    disabled={user_info.loading || user_update.loading}>
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
