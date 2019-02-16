use yew::prelude::*;

use crate::application::{
    Action,
    context::{ self, ContextAgent },
    state::State,
};

const MAX_HANDLE_LENGTH: usize = 20;

// Declare what state keys will be used by this component
interface_getstates!("handle_taken", "first_name");

interface_component!(Settings);

// This will be mapped to Settings.local:
pub struct Local {
    use_handle_text: String,
}

impl Local {
    fn new() -> Self {
        Self {
            use_handle_text: String::new(),
        }
    }
}

pub enum LocalMsg {
    UpdateHandleText(InputData),
    OnHandleSubmit,
    Ignore,
}

impl Settings {
    fn local_update(&mut self, msg: LocalMsg) -> ShouldRender {
        match msg {
            LocalMsg::UpdateHandleText(input) => {
                self.local.use_handle_text = input.value;
                return true;
            }

            LocalMsg::OnHandleSubmit => {
                self.on_handle_submit();
            }

            LocalMsg::Ignore => (),
        }
        false
    }

    fn use_handle(&mut self, handle: &str) {
        self.update(Action::UseHandle(handle.into()).into());
    }

    fn set_first_name(&mut self, first_name: &str) {
        self.update(Action::SetFirstName(first_name.into()).into());
    }

    fn on_handle_submit(&mut self) {
        let use_handle_text = self.local.use_handle_text.clone();

        // empty string given as input
        if use_handle_text.is_empty() {
            return;
        };

        // max characters exceeded
        if use_handle_text.len() > MAX_HANDLE_LENGTH {
            self.local.use_handle_text = String::new();
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
        if self.getstate.is_empty() {
            return html! { <></> };
        };
        let use_handle_text = self.local.use_handle_text.clone();
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
                                oninput=|input| LocalMsg::UpdateHandleText(input).into(),
                                onkeypress=|pressed| {
                                    if pressed.key() == "Enter" { LocalMsg::OnHandleSubmit.into() }
                                    else { LocalMsg::Ignore.into() }
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
                            onclick=|_| LocalMsg::OnHandleSubmit.into(),
                        >
                            {"Set Handle"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
