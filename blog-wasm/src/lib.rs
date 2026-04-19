#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;

mod app;
mod api;
mod types;
mod components;

pub use app::BlogApp;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    yew::Renderer::<BlogApp>::new().render();
}