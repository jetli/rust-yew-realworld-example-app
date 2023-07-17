mod banner;
mod main_view;
mod tags;

use yew::prelude::*;

use banner::Banner;
use main_view::MainView;
use tags::Tags;

/// Home page with an article list and a tag list.
#[function_component(Home)]
pub fn home() -> Html {
    let tag = use_state(|| None);
    let callback = {
        let tag = tag.clone();
        Callback::from(move |t| {
            tag.set(Some(t));
        })
    };

    html! {
        <div class="home-page">
            <Banner />
            <div class="container page">
                <div class="row">
                    <MainView tag={(*tag).clone()} />
                    <div class="col-md-3 col-xs-12">
                        <div class="sidebar">
                            <p>{ "Popular Tags" }</p>
                            <Tags {callback} />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
