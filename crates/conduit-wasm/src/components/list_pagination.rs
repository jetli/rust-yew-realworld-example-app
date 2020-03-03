use yew::{html, Callback, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};

const ITEMS_PER_PAGE: u32 = 10;

/// Pagination component
pub struct ListPagination {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub articles_count: u32,
    pub current_page: u32,
    pub callback: Callback<u32>,
}

pub enum Msg {
    PaginationChanged(u32),
}

impl Component for ListPagination {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ListPagination { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PaginationChanged(page) => {
                self.props.callback.emit(page);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        if self.props.articles_count < ITEMS_PER_PAGE {
            return html! {};
        }

        // Calculate page numbers
        let max_page = (self.props.articles_count as f32 / 10.0).ceil() as u32;
        let mut pages: Vec<u32> = vec![];
        for page in 0..max_page {
            pages.push(page);
        }

        html! {
            <nav>
                <ul class="pagination">
                {for pages.iter().map(|page| {
                    let is_current = page == &self.props.current_page;
                    let page_item_class = if is_current {
                        "page-item active"
                    } else {
                        "page-item"
                    };
                    let page = page.clone();
                    let onclick = self.link.callback(move |ev: MouseEvent| {ev.prevent_default(); Msg::PaginationChanged(page)});
                    html! {
                        <li
                            class=page_item_class
                            onclick=onclick>
                            <a class="page-link" href="">{page + 1}</a>

                        </li>
                    }
                })}
                </ul>
            </nav>
        }
    }
}
