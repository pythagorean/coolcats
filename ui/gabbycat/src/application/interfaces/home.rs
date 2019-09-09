use lazy_static::*;
use std::collections::HashMap;
use yew::prelude::*;

use super::components::autosuggest_textarea::AutosuggestTextarea;
use super::UsesLocaleValues;
use crate::application::resources;

use_locale_values!["compose_form-placeholder"];

pub struct Home {
    resources: Box<dyn Bridge<resources::Worker>>,
    locale_values: HashMap<String, String>,
}

impl Renderable<Home> for Home {
    fn view(&self) -> Html<Self> {
        let locale_value = |message_id| self.get_locale_value(message_id);
        let placeholder = locale_value("compose_form-placeholder");

        html! {
            <AutosuggestTextarea placeholder = placeholder/>
        }
    }
}

pub enum Msg {
    Resources(resources::Response),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut component = Self {
            resources: resources::Worker::bridge(link.send_back(Msg::Resources)),
            locale_values: HashMap::new(),
        };
        component.request_locale_values();
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

impl UsesLocaleValues for Home {
    fn request_locale_values(&mut self) {
        self.resources
            .send(resources::Request::LocaleValues(using_locale_values()));
    }

    fn get_locale_value(&self, message_id: &str) -> &String {
        lazy_static! {
            static ref EMPTY: String = String::from("EMPTY");
        }
        self.locale_values.get(message_id).unwrap_or(&EMPTY)
    }
}
