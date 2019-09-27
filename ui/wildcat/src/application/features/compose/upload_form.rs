use yew::prelude::*;

use crate::application::{context, state::State};
use wildcat_macros::{StateComponent, UsesStateValues, use_state_values};
use super::{sensitive_button::SensitiveButton, upload::Upload, upload_progress::UploadProgress};

use_state_values!("media_attachments");

#[derive(UsesStateValues, StateComponent)]
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