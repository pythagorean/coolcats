use stdweb::web::{
    IParentNode,
    document,
    window,
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

fn get_port() -> u16 {
    window().location().unwrap().port().unwrap().parse().unwrap()
}

fn main() {
    yew::initialize();
    let mut to_holoclient = mount_new_app("#holoclient");
    let mut to_application = mount_new_app("#application");
    to_holoclient.send_message(Msg::SetServerPort(get_port() + 888));
    to_holoclient.send_message(Msg::SetModel(ModelType::Holoclient, to_application.clone()));
    to_application.send_message(Msg::SetModel(ModelType::Application, to_holoclient.clone()));
    yew::run_loop();
}
