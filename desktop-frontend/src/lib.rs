#![recursion_limit="1024"]
mod app;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}
