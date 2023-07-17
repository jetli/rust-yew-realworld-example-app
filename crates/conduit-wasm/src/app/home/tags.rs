use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::services::tags::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub callback: Callback<String>,
}

/// A tag list component with a callback to notify that some tag is clicked.
#[function_component(Tags)]
pub fn tags(props: &Props) -> Html {
    let tag_list = use_async_with_options(
        async move { get_all().await },
        UseAsyncOptions::enable_auto(),
    );

    if let Some(tag_list) = &tag_list.data {
        html! {
            <div class="tag-list">
                {for tag_list.tags.iter().map(|tag| {
                    let onclick = {
                        let tag = tag.clone();
                        let callback = props.callback.clone();
                        Callback::from(
                            move |e: MouseEvent| {
                                e.prevent_default();
                                callback.emit(tag.clone());
                            }
                        )
                    };
                    html! {
                        <a
                            href=""
                            class="tag-default tag-pill"
                            {onclick}>
                            { &tag }
                        </a>
                    }
                })}
            </div>
        }
    } else {
        html! {
            <div>{ "Loading Tags..." }</div>
        }
    }
}
