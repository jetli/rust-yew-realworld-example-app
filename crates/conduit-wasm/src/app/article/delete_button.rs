use yew::prelude::*;
use yew_hooks::prelude::*;

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
    let delete_comment = {
        let slug = props.slug.clone();
        let comment_id = props.comment_id;
        use_async(async move { delete(slug, comment_id).await })
    };
    let onclick = {
        let delete_comment = delete_comment.clone();
        Callback::from(move |_| {
            delete_comment.run();
        })
    };

    {
        use_effect_with(
            (props.callback.clone(), props.comment_id, delete_comment),
            move |(callback, comment_id, delete_comment)| {
                if delete_comment.data.is_some() {
                    callback.emit(*comment_id);
                }
                || ()
            },
        )
    }

    html! {
        <span class="mod-options">
            <i class="ion-trash-a" {onclick} ></i>
        </span>
    }
}
