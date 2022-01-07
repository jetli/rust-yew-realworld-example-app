use std::fmt;
use std::ops::Deref;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::services::set_token;
use crate::types::UserInfo;

/// State handle for the [`use_user_context`] hook.
pub struct UseUserConextHandle {
    inner: UseStateHandle<UserInfo>,
    history: AnyHistory,
}

impl UseUserConextHandle {
    pub fn login(&self, value: UserInfo) {
        // Set global token after logged in
        set_token(Some(value.token.clone()));
        self.inner.set(value);
        // Redirect to home page
        self.history.push(AppRoute::Home);
    }

    pub fn logout(&self) {
        // Clear global token after logged out
        set_token(None);
        self.inner.set(UserInfo::default());
        // Redirect to home page
        self.history.push(AppRoute::Home);
    }
}

impl Deref for UseUserConextHandle {
    type Target = UserInfo;

    fn deref(&self) -> &Self::Target {
        &(*self.inner)
    }
}

impl Clone for UseUserConextHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            history: self.history.clone(),
        }
    }
}

impl PartialEq for UseUserConextHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

impl fmt::Debug for UseUserConextHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseUserConextHandle")
            .field("value", &format!("{:?}", *self.inner))
            .finish()
    }
}

/// This hook is used to manage user context.
pub fn use_user_context() -> UseUserConextHandle {
    let inner = use_context::<UseStateHandle<UserInfo>>().unwrap();
    let history = use_history().unwrap();

    UseUserConextHandle { inner, history }
}
