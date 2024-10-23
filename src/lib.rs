pub mod error_template;
mod objects;
pub mod web;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::web::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
