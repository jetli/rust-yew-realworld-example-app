mod banner;
mod main_view;
mod tags;

use yew::{html, Component, ComponentLink, Html, ShouldRender};

use banner::Banner;
use main_view::MainView;
use tags::Tags;

pub struct Home {}

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Home {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="home-page">
                <Banner />
                <div class="container page">
                    <div class="row">
                        <MainView />
                        <div class="col-md-3">
                            <div class="sidebar">
                                <p>{ "Popular Tags" }</p>
                                <Tags />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
