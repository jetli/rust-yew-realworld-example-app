use log::info;
use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};

use crate::agent::Tags as TagsAgent;
use crate::error::Error;
use crate::types::TagListInfo;

pub struct Tags {
    tags: TagsAgent,
    tag_list: Option<TagListInfo>,
    tag_list_callback: Callback<Result<TagListInfo, Error>>,
    tag_list_task: Option<FetchTask>,
}

pub enum Msg {
    TagListReady(Result<TagListInfo, Error>),
}

impl Component for Tags {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        Tags {
            tags: TagsAgent::new(),
            tag_list: None,
            tag_list_callback: link.send_back(Msg::TagListReady),
            tag_list_task: None,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        let task = self.tags.get_all(self.tag_list_callback.clone());
        self.tag_list_task = Some(task);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TagListReady(Ok(tag_list)) => {
                self.tag_list = Some(tag_list);
            }
            Msg::TagListReady(Err(err)) => {
                // Can't load data
                info!("{:?}", err);
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        if let Some(tag_list) = &self.tag_list {
            html! {
                <div className="tag-list">
                    {for tag_list.tags.iter().map(|tag| {
                        html! {
                            <a
                                href=""
                                class="tag-default tag-pill"
                                key=&tag
                                >
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
