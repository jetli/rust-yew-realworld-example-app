use yew::callback::Callback;
use yew::services::fetch::FetchTask;

use super::Requests;
use crate::error::Error;
use crate::types::*;

/// Apis for authentication
#[derive(Default, Debug)]
pub struct Auth {
    requests: Requests,
}

impl Auth {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    /// Get current user info
    pub fn current(&mut self, callback: Callback<Result<UserInfoWrapper, Error>>) -> FetchTask {
        self.requests
            .get::<UserInfoWrapper>("/user".to_string(), callback)
    }

    /// Login a user
    pub fn login(
        &mut self,
        login_info: LoginInfoWrapper,
        callback: Callback<Result<UserInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests.post::<LoginInfoWrapper, UserInfoWrapper>(
            "/users/login".to_string(),
            login_info,
            callback,
        )
    }

    /// Register a new user
    pub fn register(
        &mut self,
        register_info: RegisterInfoWrapper,
        callback: Callback<Result<UserInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests.post::<RegisterInfoWrapper, UserInfoWrapper>(
            "/users".to_string(),
            register_info,
            callback,
        )
    }

    /// Save info of current user
    pub fn save(
        &mut self,
        user_update_info: UserUpdateInfoWrapper,
        callback: Callback<Result<UserInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests.put::<UserUpdateInfoWrapper, UserInfoWrapper>(
            "/user".to_string(),
            user_update_info,
            callback,
        )
    }
}
