use yew::prelude::*;

use crate::application::{context, interfaces::UsesStateValues, state::State};

use_state_values!("media_attachments");

pub struct UploadForm {
    context: Box<dyn Bridge<context::Worker>>,
    substate: State,
}

impl Renderable<UploadForm> for UploadForm {
    fn view(&self) -> Html<Self> {
        let media_ids = self.substate.strings("media_attachments");

        html! {
            <div class = "compose-form__upload-wrapper">
                /*<UploadProgressContainer />*/

                <div class = "compose-form__uploads-wrapper">
                    /*
                    {mediaIds.map(id => (
                      <UploadContainer id={id} key={id} />
                    ))}
                    */
                </div>

                {if !media_ids.is_empty() { html! {/*<SensitiveButton />*/}} else { html! {} }}
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
                context::Response::StateValues(substate) => {
                    self.substate = substate;
                    true
                }
                context::Response::LocaleValues(_) => false,
            },
        }
    }
}

impl UsesStateValues for UploadForm {
    fn request_state_values(&mut self) {
        self.context.send(context::Request::GetStateValues(using_state_values()));
    }
}
