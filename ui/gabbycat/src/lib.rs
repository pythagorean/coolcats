#![recursion_limit = "143"]
use stdweb::web::{document, IParentNode};
use wasm_bindgen::prelude::*;
use yew::{html::Scope, prelude::App};

mod application;

mod model;
use model::Model;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::initialize();
    mount_new_app("#application");
    yew::run_loop();
    Ok(())
}

fn mount_new_app(selector: &'static str) -> Scope<Model> {
    let element = document().query_selector(selector).unwrap().unwrap();
    App::new().mount(element)
}
