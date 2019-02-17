use yew::prelude::*;

use crate::application::{
    Action,
    context::{ self, ContextAgent },
    state::State,
};

pub fn getstates() -> Vec<String> {
    Vec::new()
}

interface_component!(NewMeow);

// This will be mapped to NewMeow.local:
pub struct Local {
    new_meow_text: String,
}

impl Local {
    fn new() -> Self {
        Self {
            new_meow_text: String::new(),
        }
    }
}

pub enum LocalMsg {
    NewStates,
    UpdateMeowText(InputData),
    OnSubmit,
}

impl NewMeow {
    fn local_update(&mut self, msg: LocalMsg) -> ShouldRender {
        match msg {
            LocalMsg::NewStates => (),

            LocalMsg::UpdateMeowText(input) => {
                self.local.new_meow_text = input.value;
            }

            LocalMsg::OnSubmit => {
                self.on_submit();
                self.local.new_meow_text.clear();
            }
        };
        true
    }

    fn post(&mut self, message: &str) {
        self.update(Action::Post(message.into()).into());
    }

    fn on_submit(&mut self) {
        let new_meow_text = self.local.new_meow_text.clone();
        if new_meow_text.is_empty() {
            return;
        }
        self.post(&new_meow_text);
    }
}

impl Renderable<NewMeow> for NewMeow {
    fn view(&self) -> Html<Self> {
        let new_meow_text = self.local.new_meow_text.clone();

        html! {<>
            <div class="form-group col-xs-12",>
                <textarea
                    class="form-control",
                    id="meow",
                    name="meow",
                    wrap="soft",
                    value={new_meow_text},
                    oninput=|input| LocalMsg::UpdateMeowText(input).into(),
                />
                <button
                    type="submit",
                    id="postMeow",
                    class="btn btn-primary",
                    onclick=|_| LocalMsg::OnSubmit.into(),
                >
                    {"Meow"}
                </button>
            </div>
        </>}
    }
}
