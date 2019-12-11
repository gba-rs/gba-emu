#![recursion_limit="2048"]
mod app;
mod components;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}
