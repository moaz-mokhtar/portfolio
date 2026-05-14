pub mod app;
pub mod components;
#[cfg(feature = "hydrate")]
pub mod interactions;
pub mod models;
pub mod seo;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
    crate::interactions::init();
}
