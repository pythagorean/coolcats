use yew::prelude::*;
use yew_router::{ routes, Route, RouterAgent };
use serde::{ Serialize, Deserialize };
use std::sync::Mutex;
use std::str::FromStr;
use strum::AsStaticRef;

use crate::holoclient::ToHoloclient;

use super::{
    context::{ self, ContextAgent },
    state::State,
    app::App,
    edit_profile::EditProfile,
    follow::Follow,
    ToApplication,
};

// defines RouterTarget:
routes! {
    App => "/",
    EditProfile => "/editProfile",
    Follow => "/follow",
    Error => "/error",
}

pub struct Root {
    callback: Option<Callback<ToHoloclient>>,
    state: State,
    container: String,
    child: RouterTarget,
    router: Box<Bridge<RouterAgent<()>>>,
    context: Box<Bridge<ContextAgent>>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ToRoot {
    None,
    Initialize,
    Redux(String, String),
}

#[derive(PartialEq, Clone)]
pub struct Params(pub ToRoot);

#[derive(Serialize, Deserialize)]
pub enum Action {
    GetReady,
    Redirect(String),
    //ResetState,
    UseHandle(String),
    SetFirstName(String),
    SetProfilePic(String),
}

#[derive(EnumString, AsStaticStr)]
pub enum Redux {
    GetContainer,
    UseHandle,
    AgentHandle,
    SetFirstName,
    GetFirstName,
    SetProfilePic,
    GetProfilePic,
}

pub enum Msg {
    Callback(ToHoloclient),
    Route(Route<()>),
    ChangeRoute(RouterTarget),
    Action(Action),
    ContextMsg(context::Response),
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

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let router = RouterAgent::bridge(link.send_back(Msg::Route));
        let context = ContextAgent::bridge(link.send_back(Msg::ContextMsg));
        let mut root = Self {
            callback: props.callback,
            state: Default::default(),
            container: String::new(),
            child: RouterTarget::App,
            router,
            context,
        };
        root.context.send(context::Request::SetRoot);
        root
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Callback(msg) => {
                if let Some(ref mut callback) = self.callback {
                    callback.emit(msg);
                }
            }

            Msg::Route(route) => {
                self.child = route.into();
                return true;
            }

            Msg::ChangeRoute(target) => {
                self.router.send(yew_router::Request::ChangeRoute(target.into()));
            }

            Msg::Action(action) => match action {
                Action::GetReady => {
                    self.get_my_handle();
                    //self.get_handles();
                    self.get_profile_pic();
                    self.get_first_name();
                    //self.interval = setInterval(self.props.getHandles, 2000)
                }

                Action::Redirect(path) => {
                    if path.as_str() == "/" {
                        self.update(Msg::ChangeRoute(RouterTarget::App));
                    }
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

                Action::SetProfilePic(profile_pic) => self.coolcats(
                    "set_profile_pic",
                    ("data", &*profile_pic),
                    Redux::SetProfilePic.as_static(),
                ),
            },

            Msg::ContextMsg(response) => {
                if let context::Response::Request(who, request) = response {
                    match *request {
                        context::Request::GetStates(keys) => {
                            let keys: Vec<_> = keys.iter().map(|s| s.as_str()).collect();
                            self.context.send(
                                (
                                    who,
                                    context::Response::GetStates(
                                        self.state.subset(keys.as_slice()),
                                    ),
                                )
                                    .into(),
                            );
                        }

                        context::Request::Action(action) => {
                            self.update(Msg::Action(action));
                        }

                        _ => (),
                    }
                }
            }
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
                        self.update(Action::GetReady.into());
                    }

                    Redux::UseHandle => {
                        if value.is_null() {
                            let error = &result["error"];
                            if error["ValidationFailed"] == "handle_in_use" {
                                self.state.set_bool("handle_taken".into(), true);
                                return true;
                            }
                        } else {
                            let me = self.state.string("me");
                            self.state.mut_dict("handles").set_string(me, value.to_string());
                            self.state.set_bool("handle_taken".into(), false);
                            self.get_my_handle();
                        }
                    }

                    Redux::AgentHandle => {
                        if !value.is_null() {
                            self.state.set_string("handle".into(), value.to_string());
                            self.state
                                .mut_dict("app_properties")
                                .set_string("Agent_Handle".into(), value.to_string());
                            return true;
                        }
                    }

                    Redux::SetFirstName => {
                        if !value.is_null() {
                            self.get_first_name();
                        }
                    }

                    Redux::GetFirstName => {
                        if !value.is_null() {
                            self.state.set_string("first_name".into(), value.to_string());
                            return true;
                        }
                    }

                    Redux::SetProfilePic => {
                        if !value.is_null() {
                            self.get_profile_pic();
                        }
                    }

                    Redux::GetProfilePic => {
                        if !value.is_null() {
                            self.state.set_string("profile_pic".into(), value.to_string());
                            return true;
                        }
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

    fn coolcats_np(&mut self, method: &str, redux: &str) {
        let no_params = ("", "");
        self.coolcats(method, no_params, redux);
    }

    fn get_my_handle(&mut self) {
        self.coolcats("app_property", ("key", "Agent_Handle"), Redux::AgentHandle.as_static());
    }

    fn get_first_name(&mut self) {
        self.coolcats_np("get_first_name", Redux::GetFirstName.as_static());
    }

    fn get_profile_pic(&mut self) {
        self.coolcats_np("get_profile_pic", Redux::GetProfilePic.as_static());
    }
}

impl RouterTarget {
    fn counter(&self) -> u32 {
        lazy_static! {
            static ref COUNTER: Mutex<u32> = Mutex::new(0);
        }
        let mut counter = COUNTER.lock().unwrap();
        *counter += 1;
        *counter
    }
}

impl Renderable<Root> for Root {
    fn view(&self) -> Html<Self> {
        self.child.view()
    }
}

impl Renderable<Root> for RouterTarget {
    fn view(&self) -> Html<Root> {
        // Send counter parameter to notify interface components of state changes
        let counter = self.counter();
        match self {
            RouterTarget::App | RouterTarget::Error => html! { <App: counter = counter,/> },
            RouterTarget::EditProfile => html! { <EditProfile: counter = counter,/> },
            RouterTarget::Follow => html! { <Follow: counter = counter,/> },
        }
    }
}
