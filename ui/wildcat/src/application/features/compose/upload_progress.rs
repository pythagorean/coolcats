use yew::prelude::*;

use crate::application::{context, state::State};
use wildcat_macros::{StateComponent, UsesStateValues, use_state_values};

use_state_values!("is_uploading", "progress");

#[derive(UsesStateValues, StateComponent)]
pub struct UploadProgress {
    context: Box<dyn Bridge<context::Worker>>,
    substate: State,
}

impl Renderable<UploadProgress> for UploadProgress {
    fn view(&self) -> Html<Self> {
        if !self.substate.set() {
            return html! {};
        }

        let active = self.substate.bool("is_uploading");

        if !active {
            return html! {};
        }

        let _progress = self.substate.integer("progress");

        html! {
            <div class = "upload-progress">
                <div class = "upload-progress__icon">
                    /*<Icon id="upload" />*/
                </div>

                <div class = "upload-progress__message">
                    /*<FormattedMessage id = "upload-progress.label", default_message = "Uploading..." />*/

                    <div class = "upload-progress__backdrop">
                        /*
                        <Motion defaultStyle={{ width: 0 }} style={{ width: spring(progress) }}>
                          {({ width }) =>
                            <div className='upload-progress__tracker' style={{ width: `${width}%` }} />
                          }
                        </Motion>
                        */
                    </div>
                </div>
            </div>
        }
    }
}
