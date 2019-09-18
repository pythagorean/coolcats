use std::collections::HashMap;
use yew::prelude::*;

use super::components::{autosuggest_textarea::AutosuggestTextarea, upload_form::UploadForm};
use gabbycat_macros::{LocaleComponent, UsesLocaleValues, use_locale_values};
use crate::application::context;

use_locale_values!["compose_form-placeholder"];

#[derive(UsesLocaleValues, LocaleComponent)]
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
                <AutosuggestTextarea placeholder = placeholder />
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
