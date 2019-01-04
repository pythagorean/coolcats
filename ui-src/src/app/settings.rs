use yew::prelude::*;

use crate::app::{
    state::State,
    Action,
};

const MAX_HANDLE_LENGTH: usize = 20;

pub const USES_STATE: [&'static str; 1] = [
    "use_handle_text",
];

pub struct Settings {
    substate: State,
    callback: Option<Callback<Action>>,
}

fn get_first_name() -> Option<String> { Some("".into()) }

fn set_first_name(name: &str) {
    js! { alert("set_first_name: " + @{name}) }
}

fn toggle_modal() {}

pub enum Msg {
    Callback(Action),
    UpdateHandleText(ChangeData),
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
    pub substate: State,
    pub callback: Option<Callback<Action>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            substate: State::unset(),
            callback: None,
        }
    }
}

impl Component for Settings {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Settings {
            substate: props.substate,
            callback: props.callback,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Callback(msg) => {
                if let Some(ref mut callback) = self.callback {
                    callback.emit(msg);
                    return false;
                }
            },

            Msg::UpdateHandleText(ChangeData::Value(handle_text)) => {
                self.update(Action::SetString("use_handle_text".into(), handle_text.into()).into());
                return false;
            },
            Msg::UpdateHandleText(_) => (),

            Msg::OnHandleSubmit => {
                self.on_handle_submit();
            },

            Msg::Ignore => (),
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.substate = props.substate;
        true
    }
}

impl Settings {
    fn use_handle(&mut self, _handle: &str) {
    }

    fn on_handle_submit(&mut self) {
        let use_handle_text = self.substate.string("use_handle_text");

        // empty string given as input
        if use_handle_text.len() == 0 { return };

        // max characters exceeded
        if use_handle_text.len() > MAX_HANDLE_LENGTH {
            self.update(Action::SetString("use_handle_text".into(), "".into()).into());
            return
        }

        self.use_handle(&use_handle_text);

        // check if a name has been set, and if not default to handle
        match get_first_name() {
            Some(ref first_name) if first_name.len() > 1 => (),
            _ => set_first_name(&use_handle_text),
        };

        toggle_modal();
    }
}

impl Renderable<Settings> for Settings {
    fn view(&self) -> Html<Self> {
        let use_handle_text = self.substate.string("use_handle_text");
        let handle_taken = false;
        html! {
            <div classname="panel panel-default",>
                <div classname="panel-body",>
                    <div style="padding-left: 30; padding-bottom: 10;",>
                        <p
                            classname="text-info",
                            style={
                                if use_handle_text.len() == 0 && handle_taken == false {
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
                            classname="text-danger",
                            style={
                                if handle_taken == true {
                                    "display: inline;"
                                } else {
                                    "display: none;"
                                }
                            },
                        >
                            {"This handle already has a home, try something else!"}
                        </p>
                    </div>
                    <div classname="col-xs-8",>
                        <div classname="form-group input-icon",>
                            <i>{"@"}</i>
                            <input
                                value=use_handle_text,
                                onchange=|input| Msg::UpdateHandleText(input),
                                onkeypress=|pressed| {
                                    if pressed.key() == "Enter" { Msg::OnHandleSubmit }
                                    else { Msg::Ignore }
                                },
                                type="text",
                                classname="form-control",
                                id="myHandle",
                                placeholder="handle",
                            />
                        </div>
                    </div>
                    <div classname="col-xs-2",>
                        <button
                            id="setHandleButton",
                            classname="btn btn-primary",
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
