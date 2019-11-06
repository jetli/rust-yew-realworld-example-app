use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInfo {
    pub username: String,
    pub bio: Option<String>,
    pub image: String,
    pub following: bool,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArticleInfo {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub favorited: bool,
    pub favorites_count: u32,
    pub author: ProfileInfo,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArticleListInfo {
    pub articles: Vec<ArticleInfo>,
    pub articles_count: u32,
}
