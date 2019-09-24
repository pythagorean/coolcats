use yew::prelude::*;

use wildcat_macros::{UsesStateValues, class_names, component_state_update, use_state_values};
use crate::application::{context, state::State};

use_state_values!("media_attachments");

#[derive(UsesStateValues)]
pub struct Upload {
    context: Box<dyn Bridge<context::Worker>>,
    substate: State,
    id: String,
    key: String,
}

impl Renderable<Upload> for Upload {
    fn view(&self) -> Html<Self> {
        let _media = self.substate.strings("media_attachments");
        let _id = &self.id;
        let _key = &self.key;

        let active = true;

        html! {
            <div class = "compose-form__upload">
                //<Motion defaultStyle={{ scale: 0.8 }} style={{ scale: spring(1, { stiffness: 180, damping: 12 }) }}>
                <div
                    class = "compose-form__upload-thumbnail",
                    //style={{ transform: `scale(${scale})`, backgroundImage: `url(${media.get('preview_url')})`, backgroundPosition: `${x}% ${y}%` }}>
                >
                    <div class = {class_names!("compose-form__upload__actions", active)}>
                        <button class = "icon-button" />
                            //<Icon id='times' />
                            //<FormattedMessage id='upload_form.undo' defaultMessage='Delete' />
                        //</button>
                        //{media.get('type') === 'image' && <button className='icon-button' onClick={this.handleFocalPointClick}>
                            //<Icon id='crosshairs' />
                            //<FormattedMessage id='upload_form.focus' defaultMessage='Crop' />
                        //</button>}
                    </div>

                    <div class = {class_names!("compose-form__upload-description", active)}>
                        <label>
                            //<span style={{ display: 'none' }}>{intl.formatMessage(messages.description)}</span>

                            <textarea
                                //placeholder={intl.formatMessage(messages.description)}
                                //value={description}
                                //maxLength={420}
                                //onFocus={this.handleInputFocus}
                                //onChange={this.handleInputChange}
                                //onBlur={this.handleInputBlur}
                                //onKeyDown={this.handleKeyDown}
                            />
                        </label>
                    </div>
                </div>
            </div>
        }
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
    pub key: String,
}

impl Component for Upload {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let context = context::Worker::bridge(link.send_back(Msg::Context));
        let substate = State::unset();
        let id = props.id;
        let key = props.key;

        let mut component = Self {
            context,
            substate,
            id,
            key,
        };
        component.request_state_values();
        component
    }

    component_state_update!();
}
