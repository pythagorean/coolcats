use yew::prelude::*;

use crate::application::{context, state::State};
use gabbycat_macros::{UsesStateValues, use_state_values};
use super::{sensitive_button::SensitiveButton, upload::Upload, upload_progress::UploadProgress};

use_state_values!("media_attachments");

#[derive(UsesStateValues)]
pub struct UploadForm {
    context: Box<dyn Bridge<context::Worker>>,
    substate: State,
}

impl Renderable<UploadForm> for UploadForm {
    fn view(&self) -> Html<Self> {
        let media_ids = self.substate.strings("media_attachments");

        html! {
            <div class = "compose-form__upload-wrapper">
                <UploadProgress />

                <div class = "compose-form__uploads-wrapper">
                    {for media_ids.into_iter().map(|id| { html! {
                        <Upload id = id, key = id />
                    }})}
                </div>

                {if !media_ids.is_empty() { html! { <SensitiveButton /> }} else { html! {} }}
            </div>
        }
    }
}

pub enum Msg {
    Context(context::Response),
}

impl Component for UploadForm {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut component = Self {
            context: context::Worker::bridge(link.send_back(Msg::Context)),
            substate: State::unset(),
        };
        component.request_state_values();
        component
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Context(response) => match response {
                context::Response::Substate(substate) => {
                    self.substate = substate;
                    true
                }
                context::Response::LocaleValues(_) => false,
            },
        }
    }
}
