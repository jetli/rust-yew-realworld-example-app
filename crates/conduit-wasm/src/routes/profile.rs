use wasm_bindgen_futures::spawn_local;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::article_list::{ArticleList, ArticleListFilter};
use crate::hooks::use_user_context;
use crate::routes::AppRoute;
use crate::services::profiles::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub username: String,
    pub tab: ProfileTab,
}

#[derive(Clone, PartialEq)]
pub enum ProfileTab {
    ByAuthor,
    FavoritedBy,
}

/// Profile for an author
#[function_component(Profile)]
pub fn profile(props: &Props) -> Html {
    let profile_info = use_state(|| None);
    let user_ctx = use_user_context();
    let is_current_user = (*user_ctx).is_authenticated() && (*user_ctx).username == props.username;

    {
        let profile_info = profile_info.clone();
        use_effect_with_deps(
            move |username| {
                let username = username.clone();
                spawn_local(async move {
                    let profile = get(username).await;
                    if let Ok(profile) = profile {
                        profile_info.set(Some(profile.profile));
                    }
                });

                || ()
            },
            props.username.clone(),
        );
    }

    let onclick = {
        let profile_info = profile_info.clone();
        Callback::from(move |_| {
            let profile_info = profile_info.clone();
            spawn_local(async move {
                if let Some(profile) = &*profile_info {
                    let username = profile.username.clone();
                    let profile = if profile.following {
                        unfollow(username).await
                    } else {
                        follow(username).await
                    };
                    if let Ok(profile) = profile {
                        profile_info.set(Some(profile.profile));
                    }
                }
            });
        })
    };

    if let Some(profile) = &*profile_info {
        let classes_tab = if props.tab == ProfileTab::ByAuthor {
            ("nav-link active", "nav-link")
        } else {
            ("nav-link", "nav-link active")
        };

        let classes_follow = if profile.following {
            "btn btn-sm action-btn btn-secondary"
        } else {
            "btn btn-sm action-btn btn-outline-secondary"
        };

        let text = if profile.following {
            "Unfollow"
        } else {
            "Follow"
        };

        html! {
            <div class="profile-page">
                <div class="user-info">
                    <div class="container">
                        <div class="row">
                            <div class="col-xs-12 col-md-10 offset-md-1">
                                <img src={ profile.image.clone() } class="user-img" alt={ profile.username.clone() } />
                                <h4>{ &profile.username }</h4>
                                <p>
                                    {
                                        if let Some(bio) = &profile.bio {
                                            html! { bio }
                                        } else {
                                            html! { }
                                    }}
                                </p>
                                {
                                    if is_current_user {
                                        html! {
                                            <Link<AppRoute>
                                                to={AppRoute::Settings}
                                                classes="btn btn-sm btn-outline-secondary action-btn">
                                                { "Edit Profile Settings" }
                                            </Link<AppRoute>>
                                        }
                                    } else {
                                        html! {
                                            <button
                                                class={classes_follow}
                                                {onclick} >
                                                { text }
                                            </button>
                                        }
                                }}
                            </div>
                        </div>
                    </div>
                </div>
                <div class="container">
                    <div class="row">
                        <div class="col-xs-12 col-md-10 offset-md-1">
                            <div class="articles-toggle">
                                <ul class="nav nav-pills outline-active">
                                    <li class="nav-item">
                                        <Link<AppRoute>
                                            classes={classes_tab.0}
                                            to={AppRoute::Profile { username: profile.username.clone() }}>
                                            { "My Articles" }
                                        </Link<AppRoute>>
                                    </li>
                                    <li class="nav-item">
                                        <Link<AppRoute>
                                            classes={classes_tab.1}
                                            to={AppRoute::ProfileFavorites { username: profile.username.clone() }}>
                                            { "Favorited Articles" }
                                        </Link<AppRoute>>
                                    </li>
                                </ul>
                            </div>
                            {
                                match props.tab {
                                    ProfileTab::ByAuthor => {
                                        html! { <ArticleList filter={ArticleListFilter::ByAuthor(profile.username.clone())} /> }
                                    }
                                    ProfileTab::FavoritedBy => {
                                        html! { <ArticleList filter={ArticleListFilter::FavoritedBy(profile.username.clone())} /> }
                                    }
                                }
                            }
                        </div>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}
