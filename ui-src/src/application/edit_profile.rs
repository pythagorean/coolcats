use yew::prelude::*;

use crate::application::{
    Action,
    context::{ self, ContextAgent },
    state::State,
};

// Declare what state keys will be used by this component
const GETSTATES: [&str; 1] = ["handle"];

pub fn getstates() -> Vec<String> {
    lazy_static! {
        static ref VS: Vec<String> = GETSTATES.iter().map(|key| key.to_string()).collect();
    }
    VS.to_vec()
}

interface_component!(EditProfile);

// This will be mapped to EditProfile.local:
pub struct Local {
    new_name_text: String,
}

impl Local {
    fn new() -> Self {
        Self {
            new_name_text: String::new(),
        }
    }
}

pub enum LocalMsg {
    UpdateNameText(InputData),
    OnSubmit,
    Ignore,
}

impl EditProfile {
    fn local_update(&mut self, msg: LocalMsg) -> ShouldRender {
        match msg {
            LocalMsg::UpdateNameText(input) => {
                self.local.new_name_text = input.value;
                return true;
            }

            LocalMsg::OnSubmit => {
                self.on_submit();
            }

            LocalMsg::Ignore => (),
        };
        false
    }

    fn on_submit(&mut self) {
        //js! {alert(@{format!("EditProfile name submitted: {}", self.local.new_name_text)})};
    }
}

impl Renderable<EditProfile> for EditProfile {
    fn view(&self) -> Html<Self> {
        if self.getstate.is_empty() {
            return html! { <></> };
        };
        let handle = self.getstate.string("handle");
        let _new_name_text = self.local.new_name_text.clone();

        html! {
            <div class="panel panel-default",>
                <div class="close",>
                    <a href="/",>{"x"}</a>
                </div>
                <div class="panel-body",>
                    <p>{"Profile"}</p>
                    <div class="form-row",>
                        <div class="form-group col-xs-6",>
                            <label>{"Handle"}</label>
                            <p id="handle",>{"@"}{handle}</p>
                        </div>
                        <div class="form-group col-xs-6",>
                            <label>{"Name"}</label>
                            <input
                                type="text",
                                class="form-control",
                                id="inputName",
                                placeholder="name",
                                oninput=|input| LocalMsg::UpdateNameText(input).into(),
                                onkeypress=|pressed| {
                                    if pressed.key() == "Enter" { LocalMsg::OnSubmit.into() }
                                    else { LocalMsg::Ignore.into() }
                                },
                            />
                        </div>
                        <div class="form-group",>
                            <div class="form-group col-xs-10",>
                                <label>{"Profile Picture"}</label>
                                <input
                                    type="file",
                                    accept="image/*",
                                    id="image",
                                />
                            </div>
                        </div>
                    </div>
                    <div class="form-group col-xs-6",>
                        <button
                            id="saveChanges",
                            class="btn btn-primary",
                            onclick=|_| LocalMsg::OnSubmit.into(),
                        >
                            {"Save Changes"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
