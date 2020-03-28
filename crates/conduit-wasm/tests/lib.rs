use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use conduit_wasm::app::App as YewApp;
use yew::App;

#[wasm_bindgen_test]
fn home_page_has_articles() {
    let app: App<YewApp> = yew::App::new();
    app.mount(yew::utils::document().get_element_by_id("output").unwrap());

    let articles = yew::utils::document().get_elements_by_class_name("article-preview");
    assert!(articles.length() > 0);
}
