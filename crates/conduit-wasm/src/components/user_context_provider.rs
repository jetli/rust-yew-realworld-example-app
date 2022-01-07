//! User context provider.
use wasm_bindgen_futures::spawn_local;

use yew::prelude::*;

use crate::error::Error;
use crate::services::{auth::*, get_token, set_token};
use crate::types::UserInfo;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

/// User context provider.
#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = use_state(UserInfo::default);

    {
        let user_ctx = user_ctx.clone();

        use_effect_with_deps(
            move |_| {
                if get_token().is_some() {
                    spawn_local(async move {
                        let current_user = current().await;
                        match current_user {
                            Ok(user_info) => user_ctx.set(user_info.user),
                            Err(e) => match e {
                                Error::Unauthorized | Error::Forbidden => set_token(None),
                                _ => (),
                            },
                        }
                    });
                }

                || {}
            },
            (),
        );
    }

    html! {
        <ContextProvider<UseStateHandle<UserInfo>> context={user_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}
