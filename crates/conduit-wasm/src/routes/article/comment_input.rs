use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;

use yew::prelude::*;

use crate::components::list_errors::ListErrors;
use crate::hooks::use_user_context;
use crate::services::comments::*;
use crate::types::{CommentCreateInfo, CommentCreateInfoWrapper, CommentInfo};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub slug: String,
    pub callback: Callback<CommentInfo>,
}

/// Creat a comment for an article.
#[function_component(CommentInput)]
pub fn comment_input(props: &Props) -> Html {
    let create_info = use_state(CommentCreateInfo::default);
    let error = use_state(|| None);
    let user_ctx = use_user_context();

    let onsubmit = {
        let error = error.clone();
        let create_info = create_info.clone();
        let slug = props.slug.clone();
        let callback = props.callback.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default(); /* Prevent event propagation */
            let request = CommentCreateInfoWrapper {
                comment: (*create_info).clone(),
            };

            let slug = slug.clone();
            let error = error.clone();
            let callback = callback.clone();
            let create_info = create_info.clone();
            spawn_local(async move {
                let comment_info = create(slug, request).await;
                match comment_info {
                    Ok(comment_info) => {
                        error.set(None);
                        create_info.set(CommentCreateInfo::default());
                        callback.emit(comment_info.comment);
                    }
                    Err(e) => error.set(Some(e)),
                }
            });
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

    html! {
        <>
            <ListErrors error={(*error).clone()} />
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
