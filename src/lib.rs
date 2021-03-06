#![recursion_limit = "512"]

mod app;

mod components {
    pub mod button;
    pub mod header;
    pub mod menu {
      pub mod menu_item;
      pub mod menu;
    }
}
//----CompList----
use crate::components::button::Button;
use crate::components::header::Header; // HeaderComp
use crate::components::menu::menu::Menu;
use crate::components::menu::menu_item::MenuItem;

//----CompList----



use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
    Ok(())
}
