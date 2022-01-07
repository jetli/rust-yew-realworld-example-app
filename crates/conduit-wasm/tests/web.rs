mod common;

use common::timeout::Timeout;
use std::time::Duration;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use conduit_wasm::app::App as YewApp;
use yew::start_app;

#[wasm_bindgen_test]
async fn home_page_has_articles() {
    start_app::<YewApp>();

    let articles = gloo_utils::document().get_elements_by_class_name("article-preview");

    console_log!("Initial articles length: {}", articles.length());
    assert_eq!(articles.length(), 1);

    console_log!("Waiting for articles to load.");
    Timeout::new(Duration::new(10, 0)).await;

    // console_log!("Loaded articles length: {}", articles.length());
    // assert_eq!(articles.length(), 10);
}
