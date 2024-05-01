use yew::prelude::*;
use yew_router::prelude::*;

use super::delete_button::DeleteButton;
use crate::app::AppRoute;
use crate::hooks::use_user_context;
use crate::types::CommentInfo;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub slug: String,
    pub comment: CommentInfo,
    pub callback: Callback<u32>,
}

#[function_component]
pub fn Comment(props: &Props) -> Html {
    let user_ctx = use_user_context();
    let comment = &props.comment;
    let show = user_ctx.is_authenticated() && user_ctx.username == comment.author.username;

    html! {
        <div class="card">
            <div class="card-block">
                <p class="card-text">{ &comment.body }</p>
            </div>
            <div class="card-footer">
                <span class="comment-author">
                    <img src={ comment.author.image.clone() } class="comment-author-img" alt={ comment.author.username.clone() } />
                </span>
                { " " }
                <Link<AppRoute> to={AppRoute::Profile { username: comment.author.username.clone() }} classes="comment-author">
                    { &comment.author.username }
                </Link<AppRoute>>
                <span class="date-posted">
                    { &comment.created_at.format("%B %e, %Y").to_string() }
                </span>
                { if show {
                    html! {
                        <DeleteButton
                            slug={props.slug.clone()}
                            comment_id={comment.id}
                            callback={props.callback.clone()}
                            />
                    }
                } else {
                    html! { }
                }}
            </div>
        </div>
    }
}
