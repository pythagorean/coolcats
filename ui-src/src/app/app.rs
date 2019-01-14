use yew::prelude::*;

use crate::holoclient::ToHoloclient;

use super::{
    state::State,
    components::modal,
    settings::{ self, Settings },
};

const DEFAULT_PROFILE_PIC: &str = "/cat-eating-bird-circle.png";

pub struct App {
    callback: Option<Callback<ToHoloclient>>,
    onresult: Redux,
    state: State,
    container: String,
}

pub enum Action {
    GetReady,
    //ResetState,
    //ToggleModal,
    UseHandle(String),
    SetFirstName(String),
}

pub enum Redux {
    None,
    GetContainer,
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

#[derive(PartialEq, Clone, Debug)]
pub enum ToApp {
    None,
    Initialize,
    Result(String),
    Redux(String, String),
}

#[derive(PartialEq, Clone)]
pub struct Params(pub ToApp);

#[derive(PartialEq, Clone)]
pub struct Props {
    pub params: Params,
    pub callback: Option<Callback<ToHoloclient>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            params: Params(ToApp::None),
            callback: None,
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            callback: props.callback,
            onresult: Redux::None,
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
                return false;
            },

            Msg::Action(action) => match action {
                Action::GetReady => {
                    // this fetches the hash which represents the active users userHash
                    self.get_my_handle();
                    //self.get_handles();
                    //self.get_profile_pic();
                    //self.get_first_name();
                    //self.interval = setInterval(self.props.getHandles, 2000)
                },

                //Action::ResetState => {
                //    self.state = Default::default();
                //},

                //Action::ToggleModal => {
                //    if let Some(modal_is_open) = self.state.bool("modal_is_open") {
                //        self.state.set_bool("modal_is_open".into(), !modal_is_open);
                //    }
                //}

                Action::UseHandle(handle) => {
                    self.coolcats("use_handle", ("handle", &*handle), "Use_Handle");
                }

                Action::SetFirstName(first_name) => {
                    js! { alert(@{
                        format! { "SetFirstName('{}')", first_name }
                    })};
                    self.state.set_string("first_name".into(), first_name)
                }
            },
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let holoclient_msg = props.params.0;
        match holoclient_msg {
            ToApp::Initialize => {
                self.onresult = Redux::GetContainer;
                self.update(ToHoloclient::Call(
                    "info/instances".into()
                ).into());
            },

            ToApp::Result(result) => {
                let result = &json::parse(&result).unwrap();
                match self.onresult {
                    Redux::GetContainer => {
                        let container = result.entries().next().unwrap().0;
                        self.container = container.to_string();
                        self.update(Action::GetReady.into());
                    },

                    Redux::None => {
                        js! { alert(@{
                            format!("NoRedux: {}", result)
                        })}
                    },
                }
                self.onresult = Redux::None;
            },

            ToApp::Redux(result, redux) => {
                let result = &json::parse(&result).unwrap();
                let value = &result["value"];
                match redux.as_str() {
                    "Agent_Handle" => {
                        let mut app_properties = self.state.get_dict("app_properties");
                        app_properties.set_string(
                            "Agent_Handle".into(), value.to_string()
                        );
                        self.state.set_dict("app_properties".into(), app_properties);
                        self.state.set_string(
                            "handle".into(), value.to_string()
                        );
                        return true;
                    },

                    "Use_Handle" => {
                        if value.is_null() {
                            let error = &result["error"];
                            if error["ValidationFailed"] == "handle_in_use" {
                                self.state.set_bool(
                                    "handle_taken".into(), true
                                );
                            } else {
                                panic!("Redux::UseHandle error: {}", error.to_string());
                            }
                        } else {
                            let mut handles = self.state.get_dict("handles");
                            handles.set_string(
                                self.state.string("me"), value.to_string()
                            );
                            self.state.set_dict("handles".into(), handles);
                            self.state.set_string(
                                "handle".into(), value.to_string()
                            );
                            self.state.set_bool(
                                "handle_taken".into(), false
                            );
                        }
                        return true;
                    }

                    _ => {
                        js! { alert(@{
                            format!("ToApp::Redux({}, {})", result, redux)
                        })}
                    }
                }
            },

            ToApp::None => (),
        }
        false
    }
}

impl App {
    fn coolcats(&mut self, method: &str, params: (&str, &str), redux: &str) {
        let call = ToHoloclient::Call((
            vec![self.container.as_str(), "coolcats", "main", method].as_slice(),
            params,
            redux
        ).into());
        self.update(call.into());;
    }

    fn get_my_handle(&mut self) {
        self.coolcats("app_property", ("name", "Agent_Handle"), "Agent_Handle");
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        let app_properties = self.state.get_dict("app_properties");
        let modal_is_open = self.state.bool("modal_is_open");
        let profile_pic = self.state.string("profile_pic");

        if modal_is_open.unwrap() && app_properties.string("Agent_Handle").len() == 0 {
            html! {
                <div style={ modal::BACKDROP_STYLE },>
                    <div style={ modal::MODAL_STYLE },>
                        <div align="center",>
                            <p classname="h1",>{ "Welcome to Coolcats2!" }</p>
                        </div>
                        <Settings:
                            getstate = self.state.subset(settings::getstates()),
                            callback = |action| Msg::Action(action),
                        />
                    </div>
                </div>
            }
        } else {
            html! {
                <div classname="container",>
                    <div classname="spinner transition500",/>
                    <div classname="error transition500",/>
                    <div classname="row first",>
                        <div classname="fixed-area",>
                            <div classname="col-sm-2 contentcontainer",>
                                <div classname="logo",>
                                    <img
                                        src={
                                            if !profile_pic.is_empty() { &profile_pic }
                                            else { DEFAULT_PROFILE_PIC }
                                        },
                                        alt="user-profile",
                                    />
                                </div>
                                <div id="displayName",>
                                    { &format!("show: {}", true) }
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
    }
}
