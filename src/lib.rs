#![feature(path_add_extension)]
#![feature(result_flattening)]
#![feature(try_blocks)]
#![allow(warnings)]

mod objects;
pub mod web;

#[cfg(feature = "ssr")]
mod config;
#[cfg(feature = "ssr")]
mod context;
#[cfg(feature = "ssr")]
mod jobs;
#[cfg(feature = "ssr")]
mod services;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::web::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
