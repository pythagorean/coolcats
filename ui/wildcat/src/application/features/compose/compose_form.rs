use std::collections::HashMap;
use yew::prelude::*;

use mammoth_macros::{UsesLocaleValues, class_names, component_locale_update, use_locale_values};
use crate::application::{context, facilities::autosuggest_textarea::AutosuggestTextarea};
use super::upload_form::UploadForm;

use_locale_values!["compose_form-placeholder"];

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub should_condense: bool,
    #[props(required)]
    pub auto_focus: bool,
}

#[derive(UsesLocaleValues)]
pub struct ComposeForm {
    context: Box<dyn Bridge<context::Worker>>,
    locale_values: HashMap<String, String>,
    props: Props,
}

impl Renderable<ComposeForm> for ComposeForm {
    fn view(&self) -> Html<Self> {
        let locale_value = |message_id| self.get_locale_value(message_id);
        let placeholder = locale_value("compose_form-placeholder");
        if placeholder.is_empty() {
            return html! {};
        }

        let should_condense = self.props.should_condense;
        let auto_focus = self.props.auto_focus;
        let condensed = should_condense; // && !this.props.text && !this.state.composeFocused;
        let should_auto_focus = auto_focus; // && !showSearch && !isMobile(window.innerWidth)
        let edit = false;

        let compose_class_names = class_names!("compose-form", condensed);

        html! {
            <div class = compose_class_names /*ref={this.setForm} onClick={this.handleClick}*/>
                /*<Warning />*/

                {if !should_condense { html! {/*<ReplyIndicatorContainer />*/} } else { html! {} }}

                <div class="compose-form__autosuggest-wrapper", key="compose-form__autosuggest-wrapper">
                    <AutosuggestTextarea
                        placeholder = placeholder,
                        auto_focus = should_auto_focus
                    />
                    {if !condensed { html! {
                        <div class = "compose-form__modifiers">
                            <UploadForm />
                            {if !edit { html! {/*<PollForm />*/}} else { html! {} }}
                        </div>
                    }} else { html! {} }}
                </div>
            </div>
        }
    }
}

pub enum Msg {
    Context(context::Response),
}

impl Component for ComposeForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let context = context::Worker::bridge(link.send_back(Msg::Context));
        let locale_values = HashMap::new();

        let mut component = Self {
            context,
            locale_values,
            props,
        };
        component.request_locale_values(using_locale_values());
        component
    }

    component_locale_update!();
}
