use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::list_errors::ListErrors;
use crate::routes::AppRoute;
use crate::services::articles::*;
use crate::types::{ArticleCreateUpdateInfo, ArticleCreateUpdateInfoWrapper};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub slug: Option<String>,
}

/// Create or update an article
#[function_component(Editor)]
pub fn editor(props: &Props) -> Html {
    let history = use_history().unwrap();
    let error = use_state(|| None);
    let update_info = use_state(ArticleCreateUpdateInfo::default);
    let tag_input = use_state(String::default);

    {
        let update_info = update_info.clone();
        let error = error.clone();
        use_effect_with_deps(
            move |slug| {
                if let Some(slug) = slug {
                    let slug = slug.clone();
                    spawn_local(async move {
                        let article_info = get(slug).await;
                        match article_info {
                            Ok(article_info) => {
                                update_info.set(ArticleCreateUpdateInfo {
                                    title: article_info.article.title,
                                    description: article_info.article.description,
                                    body: article_info.article.body,
                                    tag_list: Some(article_info.article.tag_list),
                                });
                                error.set(None);
                            }
                            Err(e) => error.set(Some(e)),
                        }
                    });
                }

                || ()
            },
            props.slug.clone(),
        );
    }

    let onsubmit = {
        let error = error.clone();
        let update_info = update_info.clone();
        let slug = props.slug.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default(); /* Prevent event propagation */
            let request = ArticleCreateUpdateInfoWrapper {
                article: (*update_info).clone(),
            };
            let history = history.clone();
            let error = error.clone();
            let slug = slug.clone();
            spawn_local(async move {
                let article_info = {
                    if let Some(slug) = slug {
                        update(slug, request).await
                    } else {
                        create(request).await
                    }
                };
                match article_info {
                    Ok(article_info) => {
                        error.set(None);
                        // Route to home page after logged in
                        history.push(AppRoute::Article {
                            slug: article_info.article.slug,
                        });
                    }
                    Err(e) => error.set(Some(e)),
                }
            });
        })
    };
    let oninput_title = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.title = input.value();
            update_info.set(info);
        })
    };
    let oninput_description = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.description = input.value();
            update_info.set(info);
        })
    };
    let oninput_body = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.body = input.value();
            update_info.set(info);
        })
    };
    let oninput_tag = {
        let tag_input = tag_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            tag_input.set(input.value());
        })
    };
    let onkeypress = Callback::from(|e: KeyboardEvent| {
        // Prevent submit the form when press Enter
        if e.key() == "Enter" {
            e.prevent_default();
        }
    });
    let onkeyup = {
        let update_info = update_info.clone();
        let tag_input = tag_input.clone();
        Callback::from(move |e: KeyboardEvent| {
            // Add a new tag when press Enter
            if e.key() == "Enter" {
                e.prevent_default();
                // Add a new tag
                let mut info = (*update_info).clone();
                if let Some(tag_list) = &mut info.tag_list {
                    if !tag_list.contains(&*tag_input) {
                        tag_list.push((*tag_input).clone());
                    }
                } else {
                    info.tag_list = Some(vec![(*tag_input).clone()]);
                }
                update_info.set(info);
                // Clear tag input
                tag_input.set(String::default());
            }
        })
    };

    html! {
        <div class="editor-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-10 offset-md-1 col-xs-12">
                        <ListErrors error={(*error).clone()} />
                        <form {onsubmit}>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="text"
                                        placeholder="Article Title"
                                        value={update_info.title.clone()}
                                        oninput={oninput_title} />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control"
                                        type="text"
                                        placeholder="What's this article about?"
                                        value={update_info.description.clone()}
                                        oninput={oninput_description} />
                                </fieldset>
                                <fieldset class="form-group">
                                    <textarea
                                        class="form-control"
                                        rows="8"
                                        placeholder="Write your article (in markdown)"
                                        value={update_info.body.clone()}
                                        oninput={oninput_body} >
                                    </textarea>
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control"
                                        type="text"
                                        placeholder="Enter tags"
                                        value={(*tag_input).clone()}
                                        oninput={oninput_tag}
                                        {onkeypress}
                                        {onkeyup} />
                                    <div class="tag-list">
                                        {
                                            if let Some(tag_list) = &update_info.tag_list.clone() {
                                                html! {for tag_list.iter().map(|tag| {
                                                    let onclick_remove = {
                                                        let tag = tag.clone();
                                                        let update_info = update_info.clone();
                                                        Callback::from(move |_| {
                                                            // Remove a tag
                                                            let mut info = (*update_info).clone();
                                                            if let Some(tag_list) = &mut info.tag_list {
                                                                tag_list.retain(|t| t != &tag);
                                                            }
                                                            update_info.set(info);
                                                        })
                                                    };
                                                    html! {
                                                        <span class="tag-default tag-pill">
                                                            <i class="ion-close-round"
                                                                onclick={onclick_remove}>
                                                            </i>
                                                            { &tag }
                                                        </span>
                                                    }
                                                })}
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>
                                </fieldset>
                                <button
                                    class="btn btn-lg pull-xs-right btn-primary"
                                    type="submit"
                                    disabled=false>
                                    { "Publish Article" }
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
