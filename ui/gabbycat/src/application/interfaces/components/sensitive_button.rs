use std::collections::HashMap;
use yew::prelude::*;

use gabbycat_macros::{LocaleComponent, UsesLocaleValues, use_locale_values};
use crate::application::{context, utils::class_names};

use_locale_values!["compose_form-sensitive-marked", "compose_form-sensitive-unmarked"];

#[derive(UsesLocaleValues, LocaleComponent)]
pub struct SensitiveButton {
    context: Box<dyn Bridge<context::Worker>>,
    locale_values: HashMap<String, String>,
}

impl Renderable<SensitiveButton> for SensitiveButton {
    fn view(&self) -> Html<Self> {
        let locale_value = |message_id| self.get_locale_value(message_id);
        let marked = locale_value("compose_form-sensitive-marked");
        let unmarked = locale_value("compose_form-sensitive-unmarked");

        let active = false;
        let disabled = false;
        let title = if active {
            marked
        } else {
            unmarked
        };

        html! {
            <div class = "compose-form__sensitive-button">
                <label class = {class_names(&[("icon-button", active)])}, title = title>
                    <input
                        name = "mark-sensitive",
                        type = "checkbox",
                        checked = active,
                        disabled = disabled
                    />

                    <span class = {class_names(&[("checkbox", active)])} />

                    /*
                    <FormattedMessage
                        id = "compose_form-sensitive-hide",
                        default_message = "Mark media as sensitive"
                    />
                    */
                </label>
            </div>
        }
    }
}
