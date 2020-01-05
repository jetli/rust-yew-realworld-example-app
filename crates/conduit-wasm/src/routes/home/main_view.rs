use stdweb::web::event::IEvent;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::agent::is_authenticated;
use crate::components::article_list::{ArticleList, ArticleListFilter};

/// Main content with tabs of article list for home page
pub struct MainView {
    props: Props,
    tab: Tab,
    filter: ArticleListFilter,
}

#[derive(Properties)]
pub struct Props {
    pub tag: Option<String>,
}

#[derive(Clone)]
pub enum Msg {
    TagChanged(Tab),
    Ignore,
}

#[derive(PartialEq, Clone)]
pub enum Tab {
    All,
    Feed,
    Tag,
}

impl Component for MainView {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let tab = if is_authenticated() {
            Tab::Feed
        } else {
            Tab::All
        };

        let filter = if is_authenticated() {
            ArticleListFilter::Feed
        } else {
            ArticleListFilter::All
        };

        MainView { props, tab, filter }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TagChanged(tab) => {
                self.tab = tab;
                match self.tab {
                    Tab::Feed => self.filter = ArticleListFilter::Feed,
                    Tab::All => self.filter = ArticleListFilter::All,
                    Tab::Tag => {
                        if let Some(tag) = &self.props.tag {
                            self.filter = ArticleListFilter::ByTag(tag.clone());
                        }
                    }
                }
                true
            }
            Msg::Ignore => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        self.tab = Tab::Tag;
        if let Some(tag) = &self.props.tag {
            self.filter = ArticleListFilter::ByTag(tag.clone());
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="col-md-9 col-xs-12">
                <div class="feed-toggle">
                    <ul class="nav nav-pills outline-active">
                        { self.your_feed_tab() }
                        { self.global_feed_tab() }
                        { self.tag_filter_tab() }
                    </ul>
                </div>

                <ArticleList filter=&self.filter />
            </div>
        }
    }
}

impl MainView {
    fn your_feed_tab(&self) -> Html<Self> {
        if is_authenticated() {
            let (msg, class) = self.get_tab_msg_class(Tab::Feed);

            html! {
                <li class="nav-item">
                    <a  href=""
                        class=class
                        onclick=|ev| { ev.prevent_default(); msg.clone() }>
                        { "Your Feed" }
                    </a>
                </li>
            }
        } else {
            html! {}
        }
    }

    fn global_feed_tab(&self) -> Html<Self> {
        let (msg, class) = self.get_tab_msg_class(Tab::All);

        html! {
            <li class="nav-item">
                <a
                    href=""
                    class=class
                    onclick=|ev| { ev.prevent_default(); msg.clone() }>
                    { "Global Feed" }
                </a>
            </li>
        }
    }

    fn tag_filter_tab(&self) -> Html<Self> {
        if let Some(tag) = &self.props.tag {
            let (msg, class) = self.get_tab_msg_class(Tab::Tag);

            html! {
                <li class="nav-item">
                    <a
                        href=""
                        class=class
                        onclick=|ev| { ev.prevent_default(); msg.clone() }>
                        <i class="ion-pound"></i> { &tag }
                    </a>
                </li>
            }
        } else {
            html! {}
        }
    }
}

impl MainView {
    /// Get Msg and css class for tabs
    fn get_tab_msg_class(&self, tab: Tab) -> (Msg, &str) {
        let class = if self.tab == tab {
            "nav-link active"
        } else {
            "nav-link"
        };

        let msg = if self.tab == tab {
            Msg::Ignore
        } else {
            Msg::TagChanged(tab)
        };

        (msg, class)
    }
}
