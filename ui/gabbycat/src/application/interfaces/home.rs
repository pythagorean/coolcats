use lazy_static::*;
use std::collections::HashMap;
use yew::prelude::*;

use super::components::{autosuggest_textarea::AutosuggestTextarea, upload_form::UploadForm};
use gabbycat_macros::{UsesLocaleValues, use_locale_values};
use crate::application::context;

use_locale_values!["compose_form-placeholder"];

#[derive(UsesLocaleValues)]
pub struct Home {
    context: Box<dyn Bridge<context::Worker>>,
    locale_values: HashMap<String, String>,
}

impl Renderable<Home> for Home {
    fn view(&self) -> Html<Self> {
        let locale_value = |message_id| self.get_locale_value(message_id);
        let placeholder = locale_value("compose_form-placeholder");

        if placeholder.is_empty() {
            return html! {};
        }

        let condensed = false;
        let edit = false;

        html! {
            <div class="compose-form__autosuggest-wrapper", key="compose-form__autosuggest-wrapper">
                <AutosuggestTextarea placeholder = placeholder/>
                {if !condensed { html! {
                    <div class = "compose-form__modifiers">
                        <UploadForm />
                        {if !edit { html! {/*<PollForm />*/}} else { html! {} }}
                    </div>
                }} else { html! {} }}
            </div>
        }
    }
}

pub enum Msg {
    Context(context::Response),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut component = Self {
            context: context::Worker::bridge(link.send_back(Msg::Context)),
            locale_values: HashMap::new(),
        };
        component.request_locale_values();
        component
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Context(response) => match response {
                context::Response::LocaleValues(locale_values) => {
                    self.locale_values = locale_values;
                    true
                }
                context::Response::Substate(_) => false,
            },
        }
    }
}
