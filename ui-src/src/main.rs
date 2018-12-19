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

fn mount_app(selector: &'static str, app: App<Model>) -> Scope<Model> {
    let element = document().query_selector(selector).unwrap().unwrap();
    app.mount(element)
}

fn main() {
    yew::initialize();
    let holoclient = App::new();
    let app = App::new();
    let mut to_holoclient = mount_app(".holoclient", holoclient);
    let mut to_app = mount_app(".application", app);
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
