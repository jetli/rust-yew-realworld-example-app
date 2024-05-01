//! User context provider.

use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::error::Error;
use crate::services::{auth::*, get_token, set_token};
use crate::types::UserInfo;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

/// User context provider.
#[function_component]
pub fn UserContextProvider(props: &Props) -> Html {
    let user_ctx = use_state(UserInfo::default);
    let current_user = use_async(async move { current().await });

    {
        let current_user = current_user.clone();
        use_mount(move || {
            if get_token().is_some() {
                current_user.run();
            }
        });
    }

    {
        let user_ctx = user_ctx.clone();
        use_effect_with(current_user, move |current_user| {
            if let Some(user_info) = &current_user.data {
                user_ctx.set(user_info.user.clone());
            }

            if let Some(error) = &current_user.error {
                match error {
                    Error::Unauthorized | Error::Forbidden => set_token(None),
                    _ => (),
                }
            }
            || ()
        })
    }

    html! {
        <ContextProvider<UseStateHandle<UserInfo>> context={user_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}
