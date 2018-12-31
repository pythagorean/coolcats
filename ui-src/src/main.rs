extern crate yew;
extern crate coolcats2;

use stdweb::web::{
    IParentNode,
    document,
};

use yew::{
    prelude::App,
    html::Scope,
};

use coolcats2::{
    Model,
    ModelType,
    Msg,
};

fn mount_new_app(selector: &'static str) -> Scope<Model> {
    let element = document().query_selector(selector).unwrap().unwrap();
    App::new().mount(element)
}

fn main() {
    yew::initialize();
    let mut to_holoclient = mount_new_app("#holoclient");
    let mut to_app = mount_new_app("#application");
    to_holoclient.send_message(Msg::SetModel(
        ModelType::Holoclient,
        to_app.clone()
    ));
    to_app.send_message(Msg::SetModel(
        ModelType::App,
        to_holoclient.clone()
    ));
    yew::run_loop();
}
