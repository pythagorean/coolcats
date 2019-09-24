// Should implement i18n like https://github.com/formatjs/react-intl

use std::collections::HashMap;
use yew::prelude::*;

use mammoth_macros::{UsesLocaleValues, component_locale_update};
use crate::application::context;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub id: String,
    #[props(required)]
    pub default_message: String,
}

#[derive(UsesLocaleValues)]
pub struct FormattedMessage {
    context: Box<dyn Bridge<context::Worker>>,
    locale_values: HashMap<String, String>,
    props: Props,
}

impl Renderable<FormattedMessage> for FormattedMessage {
    fn view(&self) -> Html<Self> {
        let locale_value = |message_id| self.get_locale_value(message_id);
        let mut message = locale_value(&self.props.id);
        if message.is_empty() {
            message = &self.props.default_message;
        }

        html! { {message} }
    }
}

pub enum Msg {
    Context(context::Response),
}

impl Component for FormattedMessage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let context = context::Worker::bridge(link.send_back(Msg::Context));
        let locale_values = HashMap::new();
        let message_id = props.id.clone();

        let mut component = Self {
            context,
            locale_values,
            props,
        };
        component.request_locale_values(vec![message_id]);
        component
    }

    component_locale_update!();
}
