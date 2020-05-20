mod banner;
mod main_view;
mod tags;

use yew::{html, Component, ComponentLink, Html, ShouldRender};

use banner::Banner;
use main_view::MainView;
use tags::Tags;

/// Home page with an article list and a tag list.
pub struct Home {
    tag: Option<String>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    TagFiltered(String),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Home { tag: None, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TagFiltered(tag) => {
                self.tag = Some(tag);
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let callback = self.link.callback(Msg::TagFiltered);

        html! {
            <div class="home-page">
                <Banner />
                <div class="container page">
                    <div class="row">
                        <MainView tag=&self.tag />
                        <div class="col-md-3 col-xs-12">
                            <div class="sidebar">
                                <p>{ "Popular Tags" }</p>
                                <Tags callback=callback />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
