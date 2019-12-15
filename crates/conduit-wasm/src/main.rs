use web_logger;

use conduit_wasm::App;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    web_logger::init();
    yew::start_app::<App>();
}
