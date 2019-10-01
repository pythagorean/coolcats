#![recursion_limit = "320"]
use stdweb::web::{document, IParentNode};
use wasm_bindgen::prelude::*;
use yew::{html::Scope, prelude::App};

mod application;

mod model;
use model::{Model, ModelType, Msg as ModelMsg};

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::initialize();
    let mut to_holoclient = mount_new_app("#holoclient");
    let mut to_application = mount_new_app("#application");
    to_holoclient.send_message(ModelMsg::SetModel(ModelType::Holoclient, to_application.clone()));
    to_application.send_message(ModelMsg::SetModel(ModelType::Application, to_holoclient));
    yew::run_loop();
    Ok(())
}

fn mount_new_app(selector: &'static str) -> Scope<Model> {
    let element = document().query_selector(selector).unwrap().unwrap();
    App::new().mount(element)
}
