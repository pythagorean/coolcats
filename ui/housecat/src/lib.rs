#![recursion_limit = "551"]
#[macro_use]
extern crate stdweb;
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate lazy_static;

mod application;
mod model;

use wasm_bindgen::prelude::*;
use stdweb::web::{IParentNode, document, window};
use yew::{prelude::App, html::Scope};

use model::{Model, ModelType, Msg};

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::initialize();
    let mut to_holoclient = mount_new_app("#holoclient");
    let mut to_application = mount_new_app("#application");
    to_holoclient.send_message(Msg::SetServerPort(get_port() + 888));
    to_holoclient.send_message(Msg::SetModel(ModelType::Holoclient, to_application.clone()));
    to_application.send_message(Msg::SetModel(ModelType::Application, to_holoclient.clone()));
    yew::run_loop();
    Ok(())
}

fn mount_new_app(selector: &'static str) -> Scope<Model> {
    let element = document().query_selector(selector).unwrap().unwrap();
    App::new().mount(element)
}

fn get_port() -> u16 {
    window().location().unwrap().port().unwrap().parse().unwrap()
}
