#![recursion_limit = "512"] 

mod app;
mod pages;
mod route;


use wasm_bindgen::prelude::*;
use yew::prelude::*;


#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<app::App>::new().mount_to_body();
}
