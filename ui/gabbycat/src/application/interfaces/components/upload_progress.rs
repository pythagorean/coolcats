use yew::prelude::*;

use crate::application::{context, state::State};
use gabbycat_macros::{UsesStateValues, use_state_values};

use_state_values!("is_uploading", "progress");

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

        let progress = self.substate.integer("progress");

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

pub enum Msg {
    Context(context::Response),
}

impl Component for UploadProgress {
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

impl UsesStateValues for UploadProgress {
    fn request_state_values(&mut self) {
        self.context.send(context::Request::GetSubstate(using_state_values()));
    }
}
