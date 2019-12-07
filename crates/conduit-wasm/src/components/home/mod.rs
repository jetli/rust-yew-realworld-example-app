mod banner;
mod main_view;
mod tags;

use yew::{html, Component, ComponentLink, Html, ShouldRender};

use banner::Banner;
use main_view::MainView;
use tags::Tags;

pub struct Home {
    tag: Option<String>,
}

pub enum Msg {
    TagFiltered(String),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Home { tag: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TagFiltered(tag) => {
                self.tag = Some(tag);
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="home-page">
                <Banner />
                <div class="container page">
                    <div class="row">
                        <MainView tag=&self.tag />
                        <div class="col-md-3">
                            <div class="sidebar">
                                <p>{ "Popular Tags" }</p>
                                <Tags callback=Msg::TagFiltered />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
