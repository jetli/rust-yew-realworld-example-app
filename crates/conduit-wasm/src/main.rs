use web_logger;

use conduit_wasm::App;

fn main() {
    web_logger::init();
    yew::start_app::<App>();
}
