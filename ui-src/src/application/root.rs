use yew::prelude::*;
use std::str::FromStr;
use strum::AsStaticRef;

use crate::holoclient::ToHoloclient;

use super::{
    state::State,
    app::{ self, App },
    ToApplication,
};

pub struct Root {
    callback: Option<Callback<ToHoloclient>>,
    state: State,
    container: String,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ToRoot {
    None,
    Initialize,
    Redux(String, String),
}

#[derive(PartialEq, Clone)]
pub struct Params(pub ToRoot);

pub enum Action {
    GetReady,
    //ResetState,
    UseHandle(String),
    SetFirstName(String),
}

#[derive(EnumString, AsStaticStr)]
pub enum Redux {
    GetContainer,
    UseHandle,
    AgentHandle,
    SetFirstName,
    GetFirstName,
}

pub enum Msg {
    Callback(ToHoloclient),
    Action(Action),
}

impl From<ToHoloclient> for Msg {
    fn from(msg: ToHoloclient) -> Self {
        Msg::Callback(msg)
    }
}

impl From<Action> for Msg {
    fn from(action: Action) -> Self {
        Msg::Action(action)
    }
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub params: Params,
    pub callback: Option<Callback<ToHoloclient>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            params: Params(ToRoot::None),
            callback: None,
        }
    }
}

impl Component for Root {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            callback: props.callback,
            state: Default::default(),
            container: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Callback(msg) => {
                if let Some(ref mut callback) = self.callback {
                    callback.emit(msg);
                }
            }

            Msg::Action(action) => match action {
                Action::GetReady => {
                    self.get_my_handle();
                    //self.get_handles();
                    //self.get_profile_pic();
                    self.get_first_name();
                    //self.interval = setInterval(self.props.getHandles, 2000)
                }

                //Action::ResetState => {
                //    self.state = Default::default();
                //},
                Action::UseHandle(handle) => {
                    self.coolcats("use_handle", ("handle", &*handle), Redux::UseHandle.as_static());
                }

                Action::SetFirstName(first_name) => {
                    self.coolcats(
                        "set_first_name",
                        ("name", &*first_name),
                        Redux::SetFirstName.as_static(),
                    );
                }
            },
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let holoclient_msg = props.params.0;
        match holoclient_msg {
            ToApplication::Initialize => {
                self.update(
                    ToHoloclient::Call(("info/instances", Redux::GetContainer.as_static()).into())
                        .into(),
                );
            }

            ToApplication::Redux(result, redux) => {
                let result = &json::parse(&result).unwrap();
                let redux = Redux::from_str(&redux).unwrap();
                let value = &result["value"];

                match redux {
                    Redux::GetContainer => {
                        self.container = result[0]["id"].to_string();
                        //Disabled because get_my_handle before handle is set has Zome problem
                        //self.update(Action::GetReady.into());
                    }

                    Redux::UseHandle => {
                        if value.is_null() {
                            let error = &result["error"];
                            if error["ValidationFailed"] == "handle_in_use" {
                                self.state.set_bool("handle_taken".into(), true);
                                return true;
                            } else {
                                panic!("Redux::UseHandle error: {}", error.to_string());
                            }
                        } else {
                            let me = self.state.string("me");
                            self.state.mut_dict("handles").set_string(me, value.to_string());
                            self.state.set_bool("handle_taken".into(), false);
                            self.update(Action::GetReady.into());
                        }
                    }

                    Redux::AgentHandle => {
                        self.state.set_string("handle".into(), value.to_string());
                        self.state
                            .mut_dict("app_properties")
                            .set_string("Agent_Handle".into(), value.to_string());
                        return true;
                    }

                    Redux::SetFirstName => (),

                    Redux::GetFirstName => {
                        self.state.set_string("first_name".into(), value.to_string());
                        return true;
                    }
                }
            }

            ToApplication::None => (),
        }
        false
    }
}

impl Root {
    fn coolcats(&mut self, method: &str, params: (&str, &str), redux: &str) {
        let call = ToHoloclient::Call(
            (&[self.container.as_str(), "coolcats", "main", method][..], params, redux).into(),
        );
        self.update(call.into());;
    }

    fn get_my_handle(&mut self) {
        self.coolcats("app_property", ("key", "Agent_Handle"), Redux::AgentHandle.as_static());
    }

    fn get_first_name(&mut self) {
        let no_params = ("", "");
        self.coolcats("get_first_name", no_params, Redux::GetFirstName.as_static())
    }
}

impl Renderable<Root> for Root {
    fn view(&self) -> Html<Self> {
        html! {
            <App:
                getstate = self.state.subset(app::getstates().as_slice()),
                callback = Msg::Action,
            />
        }
    }
}
