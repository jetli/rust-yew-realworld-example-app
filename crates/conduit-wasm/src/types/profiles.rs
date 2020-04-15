use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInfo {
    pub username: String,
    pub bio: Option<String>,
    pub image: String,
    pub following: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInfoWrapper {
    pub profile: ProfileInfo,
}
