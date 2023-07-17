use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::time::sleep;

wasm_bindgen_test_configure!(run_in_browser);

use conduit_wasm::app::App;

#[wasm_bindgen_test]
async fn home_page_has_articles() {
    yew::Renderer::<App>::with_root(gloo_utils::document().get_element_by_id("output").unwrap())
        .render();

    let articles = gloo_utils::document().get_elements_by_class_name("article-preview");

    console_log!("Initial articles length: {}", articles.length());
    assert_eq!(articles.length(), 0);

    console_log!("Waiting for articles to load.");
    sleep(Duration::new(5, 0)).await;

    console_log!("Loaded articles length: {}", articles.length());
    assert_eq!(articles.length(), 10);
}
