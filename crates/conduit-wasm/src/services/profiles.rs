use yew::callback::Callback;
use yew::services::fetch::FetchTask;

use super::Requests;
use crate::error::Error;
use crate::types::*;

/// Apis for profile
#[derive(Default, Debug)]
pub struct Profiles {
    requests: Requests,
}

impl Profiles {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub fn follow(
        &mut self,
        username: String,
        callback: Callback<Result<ProfileInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests.post::<(), ProfileInfoWrapper>(
            format!("/profiles/{}/follow", username),
            (),
            callback,
        )
    }

    pub fn unfollow(
        &mut self,
        username: String,
        callback: Callback<Result<ProfileInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests
            .delete::<ProfileInfoWrapper>(format!("/profiles/{}/follow", username), callback)
    }

    pub fn get(
        &mut self,
        username: String,
        callback: Callback<Result<ProfileInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests
            .get::<ProfileInfoWrapper>(format!("/profiles/{}", username), callback)
    }
}
