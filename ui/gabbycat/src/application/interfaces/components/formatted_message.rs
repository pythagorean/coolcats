// Should implement i18n like https://github.com/formatjs/react-intl

use std::collections::HashMap;
use yew::prelude::*;

use gabbycat_macros::{UsesLocaleValues, component_locale_update};
use crate::application::context;

#[derive(UsesLocaleValues)]
pub struct FormattedMessage {
    context: Box<dyn Bridge<context::Worker>>,
    locale_values: HashMap<String, String>,
    id: String,
    default_message: String,
}

impl Renderable<FormattedMessage> for FormattedMessage {
    fn view(&self) -> Html<Self> {
        let locale_value = |message_id| self.get_locale_value(message_id);
        let mut message = locale_value(&self.id);
        if message.is_empty() {
            message = &self.default_message;
        }

        html! { {message} }
    }
}

pub enum Msg {
    Context(context::Response),
}

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub id: String,
    #[props(required)]
    pub default_message: String,
}

impl Component for FormattedMessage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let context = context::Worker::bridge(link.send_back(Msg::Context));
        let locale_values = HashMap::new();
        let id = props.id;
        let default_message = props.default_message;

        let mut component = Self {
            context,
            locale_values,
            id,
            default_message,
        };
        component.request_locale_values(vec![component.id.clone()]);
        component
    }

    component_locale_update!();
}
