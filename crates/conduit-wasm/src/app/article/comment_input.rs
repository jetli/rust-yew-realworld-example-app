use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::list_errors::ListErrors;
use crate::hooks::use_user_context;
use crate::services::comments::*;
use crate::types::{CommentCreateInfo, CommentCreateInfoWrapper, CommentInfo};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub slug: String,
    pub callback: Callback<CommentInfo>,
}

/// Create a comment for an article.
#[function_component(CommentInput)]
pub fn comment_input(props: &Props) -> Html {
    let create_info = use_state(CommentCreateInfo::default);
    let user_ctx = use_user_context();
    let create_comment = {
        let request = CommentCreateInfoWrapper {
            comment: (*create_info).clone(),
        };
        let slug = props.slug.clone();
        use_async(async move { create(slug, request).await })
    };

    let onsubmit = {
        let create_comment = create_comment.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); /* Prevent event propagation */
            create_comment.run();
        })
    };
    let oninput = {
        let create_info = create_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*create_info).clone();
            info.body = input.value();
            create_info.set(info);
        })
    };

    {
        let create_info = create_info.clone();
        let callback = props.callback.clone();
        use_effect_with(
            create_comment.clone(),
            move |create_comment| {
                if let Some(comment_info) = &create_comment.data {
                    create_info.set(CommentCreateInfo::default());
                    callback.emit(comment_info.comment.clone());
                }

                || ()
            },
        );
    }

    html! {
        <>
            <ListErrors error={create_comment.error.clone()} />
            <form class="card comment-form" onsubmit={onsubmit}>
                <div class="card-block">
                    <textarea class="form-control"
                        placeholder="Write a comment..."
                        rows="3"
                        value={create_info.body.clone()}
                        oninput={oninput} >
                    </textarea>
                </div>
                <div class="card-footer">
                    {if user_ctx.is_authenticated() {
                        html! {
                            <img
                                src={ user_ctx.image.clone() }
                                class="comment-author-img"
                                alt={ user_ctx.username.clone()} />
                        }
                    } else {
                        html! { }
                    }}
                    <button
                        class="btn btn-sm btn-primary"
                        type="submit">
                        { "Post Comment" }
                    </button>
                </div>
            </form>
        </>
    }
}
