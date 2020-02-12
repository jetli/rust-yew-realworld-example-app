use yew::services::fetch::FetchTask;
use yew::{html, Callback, MouseEvent, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::agent::Tags as TagsAgent;
use crate::error::Error;
use crate::types::TagListInfo;

/// A tag list component with a callback to notify that some tag is clicked.
pub struct Tags {
    tags: TagsAgent,
    tag_list: Option<TagListInfo>,
    response: Callback<Result<TagListInfo, Error>>,
    task: Option<FetchTask>,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[props(required)]
    pub callback: Callback<String>,
}

pub enum Msg {
    Response(Result<TagListInfo, Error>),
    TagFiltered(String),
}

impl Component for Tags {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Tags {
            tags: TagsAgent::new(),
            tag_list: None,
            response: link.callback(Msg::Response),
            task: None,
            props,
            link,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        let task = self.tags.get_all(self.response.clone());
        self.task = Some(task);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(tag_list)) => {
                self.tag_list = Some(tag_list);
            }
            Msg::Response(Err(_)) => {}
            Msg::TagFiltered(tag) => {
                self.props.callback.emit(tag);
            }
        }
        true
    }

    fn view(&self) -> Html {
        if let Some(tag_list) = &self.tag_list {
            html! {
                <div className="tag-list">
                    {for tag_list.tags.iter().map(|tag| {
                        let tag_filtered = tag.clone();
                        let onclick = self.link.callback(move |ev: MouseEvent| { ev.prevent_default(); Msg::TagFiltered(tag_filtered.to_string()) });
                        html! {
                            <a
                                href=""
                                class="tag-default tag-pill"
                                onclick=onclick>
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
}
