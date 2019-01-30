use yew::prelude::*;

use crate::application::{
    state::State,
    Action,
};

const MAX_HANDLE_LENGTH: usize = 20;

// Declare what state keys will be used by this component
const GETSTATES: [&str; 2] = ["handle_taken", "first_name"];

pub fn getstates() -> Vec<&'static str> {
    GETSTATES.to_vec()
}

pub struct Settings {
    getstate: State,
    callback: Option<Callback<Action>>,

    use_handle_text: String,
}

pub enum Msg {
    Callback(Action),
    UpdateHandleText(InputData),
    OnHandleSubmit,
    Ignore,
}

impl From<Action> for Msg {
    fn from(action: Action) -> Self {
        Msg::Callback(action)
    }
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub getstate: State,
    pub callback: Option<Callback<Action>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            getstate: State::unset(),
            callback: None,
        }
    }
}

impl Component for Settings {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Settings {
            getstate: props.getstate,
            callback: props.callback,

            use_handle_text: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Callback(msg) => {
                if let Some(ref mut callback) = self.callback {
                    callback.emit(msg);
                }
            }

            Msg::UpdateHandleText(input) => {
                self.use_handle_text = input.value;
                return true;
            }

            Msg::OnHandleSubmit => {
                self.on_handle_submit();
            }

            Msg::Ignore => (),
        };
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.getstate = props.getstate;
        true
    }
}

impl Settings {
    fn use_handle(&mut self, handle: &str) {
        self.update(Action::UseHandle(handle.into()).into());
    }

    fn set_first_name(&mut self, first_name: &str) {
        self.update(Action::SetFirstName(first_name.into()).into());
    }

    fn on_handle_submit(&mut self) {
        let use_handle_text = self.use_handle_text.clone();

        // empty string given as input
        if use_handle_text.is_empty() {
            return;
        };

        // max characters exceeded
        if use_handle_text.len() > MAX_HANDLE_LENGTH {
            self.use_handle_text = String::new();
            return;
        }

        self.use_handle(&use_handle_text);

        // check if a name has been set, and if not default to handle
        if self.getstate.string("first_name").is_empty() {
            self.set_first_name(&use_handle_text);
        }
    }
}

impl Renderable<Settings> for Settings {
    fn view(&self) -> Html<Self> {
        let use_handle_text = self.use_handle_text.clone();
        let handle_taken = self.getstate.bool("handle_taken").unwrap();

        html! {
            <div class="panel panel-default",>
                <div class="panel-body",>
                    <div style="padding-left: 30; padding-bottom: 10;",>
                        <p
                            class="text-info",
                            style={
                                if use_handle_text.is_empty() && !handle_taken {
                                    "display: inline;"
                                } else {
                                    "display: none;"
                                }
                            },
                        >
                            {"Set your handle to get meowing"}
                        </p>
                    </div>
                    <div style="padding-left: 30; padding-bottom: 10;",>
                        <p
                            class="text-danger",
                            style={
                                if handle_taken {
                                    "display: inline;"
                                } else {
                                    "display: none;"
                                }
                            },
                        >
                            {"This handle already has a home, try something else!"}
                        </p>
                    </div>
                    <div class="col-xs-8",>
                        <div class="form-group input-icon",>
                            <i>{"@"}</i>
                            <input
                                value=use_handle_text,
                                oninput=|input| Msg::UpdateHandleText(input),
                                onkeypress=|pressed| {
                                    if pressed.key() == "Enter" { Msg::OnHandleSubmit }
                                    else { Msg::Ignore }
                                },
                                type="text",
                                class="form-control",
                                id="myHandle",
                                placeholder="handle",
                            />
                        </div>
                    </div>
                    <div class="col-xs-2",>
                        <button
                            id="setHandleButton",
                            class="btn btn-primary",
                            onclick=|_| Msg::OnHandleSubmit,
                        >
                            {"Set Handle"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
