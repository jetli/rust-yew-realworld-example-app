use wasm_bindgen_futures::spawn_local;

use yew::prelude::*;

use crate::services::comments::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub slug: String,
    pub comment_id: u32,
    pub callback: Callback<u32>,
}

/// A component to delete a comment from an article.
#[function_component(DeleteButton)]
pub fn delete_button(props: &Props) -> Html {
    let onclick = {
        let slug = props.slug.clone();
        let comment_id = props.comment_id;
        let callback = props.callback.clone();
        Callback::from(move |_| {
            let slug = slug.clone();
            let callback = callback.clone();
            spawn_local(async move {
                if delete(slug, comment_id).await.is_ok() {
                    callback.emit(comment_id);
                }
            });
        })
    };
    html! {
        <span class="mod-options">
            <i class="ion-trash-a" {onclick} ></i>
        </span>
    }
}
