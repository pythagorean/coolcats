use yew::prelude::*;

use std::collections::HashMap;

use crate::application::resources;

pub struct Home {
    resources: Box<dyn Bridge<resources::Worker>>,
    locale_values: HashMap<String, String>,
}

pub enum Msg {
    Resources(resources::Response),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let resources = resources::Worker::bridge(link.send_back(Msg::Resources));
        let mut component = Self {
            resources,
            locale_values: HashMap::new(),
        };
        component.initialize();
        component
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Resources(response) => match response {
                resources::Response::LocaleValues(locale_values) => {
                    self.locale_values = locale_values;
                    true
                }
            },
        }
    }
}

impl Home {
    fn initialize(&mut self) {
        self.resources.send(resources::Request::LocaleValues(vec![
            "compose_form-placeholder".into(),
        ]));
    }
}

impl Renderable<Home> for Home {
    fn view(&self) -> Html<Self> {
        let empty = String::new();
        let value = self
            .locale_values
            .get("compose_form-placeholder")
            .unwrap_or(&empty);
        html! {
            <p>{value}</p>
        }
    }
}
