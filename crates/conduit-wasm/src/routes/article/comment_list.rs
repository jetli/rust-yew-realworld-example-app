use wasm_bindgen_futures::spawn_local;

use yew::prelude::*;
use yew_router::prelude::*;

use super::comment::Comment;
use super::comment_input::CommentInput;
use crate::hooks::use_user_context;
use crate::routes::AppRoute;
use crate::services::comments::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub slug: String,
}

/// A comment list component of an article.
#[function_component(CommentList)]
pub fn comment_list(props: &Props) -> Html {
    let comment_list = use_state(|| None);
    let user_ctx = use_user_context();

    {
        let comment_list = comment_list.clone();
        use_effect_with_deps(
            move |slug| {
                let slug = slug.clone();
                spawn_local(async move {
                    if let Ok(list_info) = for_article(slug.clone()).await {
                        comment_list.set(Some(list_info.comments));
                    }
                });

                || ()
            },
            props.slug.clone(),
        );
    }

    let callback_added = {
        let comment_list = comment_list.clone();
        Callback::from(move |comment_info| {
            if let Some(mut list) = (*comment_list).clone() {
                list.insert(0, comment_info);
                comment_list.set(Some(list));
            }
        })
    };
    let callback_deleted = {
        let comment_list = comment_list.clone();
        Callback::from(move |comment_id| {
            if let Some(mut list) = (*comment_list).clone() {
                list.retain(|c| c.id != comment_id);
                comment_list.set(Some(list));
            }
        })
    };

    if let Some(comment_list) = &*comment_list {
        html! {
            <div class="col-xs-12 col-md-8 offset-md-2">
                {
                    if user_ctx.is_authenticated() {
                        html! {
                            <div>
                                <CommentInput
                                    slug={props.slug.clone()}
                                    callback={callback_added} />
                            </div>
                        }
                    } else {
                        html! {
                            <p>
                                <Link<AppRoute> to={AppRoute::Login} classes="nav-link">
                                    { "Sign in" }
                                </Link<AppRoute>>
                                { " or " }
                                <Link<AppRoute> to={AppRoute::Register} classes="nav-link">
                                    { "sign up" }
                                </Link<AppRoute>>
                                { " to add comments on this article." }
                            </p>
                        }
                    }
                }
                <div>
                    {for comment_list.iter().map(|comment| {
                        html! {
                            <Comment
                                slug={props.slug.clone()}
                                comment={comment.clone()}
                                callback={callback_deleted.clone()} />
                        }
                    })}
                </div>
            </div>
        }
    } else {
        html! {}
    }
}
