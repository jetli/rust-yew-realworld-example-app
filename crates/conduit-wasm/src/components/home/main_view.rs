use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::components::article_list::ArticleList;

pub struct MainView {}

pub enum Msg {}

impl Component for MainView {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MainView {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="col-md-9">
                <div class="feed-toggle">
                    <ul class="nav nav-pills outline-active">
                        <li className="nav-item">
                            <a
                                href=""
                                class="nav-link active"
                                >
                                { "Global Feed" }
                            </a>
                        </li>
                    </ul>
                </div>

                <ArticleList />
            </div>
        }
    }
}
