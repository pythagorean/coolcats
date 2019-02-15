use yew::prelude::*;
use yew_router::{ routes, Route, RouterAgent };
use serde::{ Serialize, Deserialize };
use std::sync::Mutex;
use std::str::FromStr;
use strum::AsStaticRef;
use stdweb::web::Date;

use crate::{
    utils::Dict,
    holoclient::ToHoloclient,
};

use super::{
    context::{ self, ContextAgent },
    state::State,
    interface::{
        app::App,
        edit_profile::EditProfile,
        follow::Follow,
    },
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
    conductor: String,
    path: String,
    child: RouterTarget,
    router: Box<Bridge<RouterAgent<()>>>,
    context: Box<Bridge<ContextAgent>>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ToRoot {
    None,
    Initialize,
    Redux(String, String, String),
}

#[derive(PartialEq, Clone)]
pub struct Params(pub ToRoot);

#[derive(Serialize, Deserialize)]
pub enum Action {
    GetReady,
    Redirect(String),
    UseHandle(String),
    SetFirstName(String),
    SetProfilePic(String),
    Post(String),
}

#[derive(EnumString, AsStaticStr)]
pub enum Redux {
    GetConductor,
    UseHandle,
    AgentHandle,
    SetFirstName,
    GetFirstName,
    SetProfilePic,
    GetProfilePic,
    Post,
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
            conductor: String::new(),
            path: String::from("/"),
            child: RouterTarget::App,
            router,
            context,
        };
        root.load_profile();
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
                if let Some(path) = route.clone().fragment {
                    self.path = path;
                }
                self.child = route.into();
                return true;
            }

            Msg::ChangeRoute(target) => {
                self.router.send(yew_router::Request::ChangeRoute(target.into()));
            }

            Msg::ContextMsg(response) => {
                if let context::Response::Request(who, request) = response {
                    match *request {
                        context::Request::GetPath => {
                            self.context
                                .send((who, context::Response::GetPath(self.path.clone())).into());
                        }

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

            Msg::Action(action) => match action {
                Action::GetReady => {
                    self.get_my_handle();
                    //self.get_handles();
                    self.get_profile_pic();
                    self.get_first_name();
                    //self.interval = setInterval(self.props.getHandles, 2000)
                }

                Action::Redirect(path) => {
                    if path.as_str() == "/#/" {
                        self.update(Msg::ChangeRoute(RouterTarget::App));
                    }
                }

                Action::UseHandle(handle) => {
                    self.coolcats(
                        "use_handle",
                        &[("handle", &*handle)],
                        Redux::UseHandle.as_static(),
                    );
                }

                Action::SetFirstName(first_name) => {
                    self.state.set_string("first_name".into(), first_name.clone());
                    self.coolcats(
                        "set_first_name",
                        &[("name", &*first_name)],
                        Redux::SetFirstName.as_static(),
                    );
                    self.save_profile();
                    return true;
                }

                Action::SetProfilePic(profile_pic) => {
                    self.state.set_string("profile_pic".into(), profile_pic.clone());
                    self.coolcats(
                        "set_profile_pic",
                        &[("dataurl", &*profile_pic)],
                        Redux::SetProfilePic.as_static(),
                    );
                    self.save_profile();
                    return true;
                }

                Action::Post(message) => {
                    let stamp = Date::now().to_string();
                    let mut post = Dict::new();
                    post.insert("author".into(), self.state.string("handle").into());
                    post.insert("message".into(), message.clone().into());
                    self.state.mut_dict("posts").insert(stamp.clone(), post.into());
                    self.coolcats_idx(
                        "post",
                        &[("message", &*message), ("stamp", &stamp)],
                        Redux::Post.as_static(),
                        &stamp,
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
                    ToHoloclient::Call(("info/instances", Redux::GetConductor.as_static()).into())
                        .into(),
                );
            }

            ToApplication::Redux(result, redux, index) => {
                let result = &json::parse(&result).unwrap();
                let redux = Redux::from_str(&redux).unwrap();
                let value = &result["value"];

                match redux {
                    Redux::GetConductor => {
                        self.conductor = result[0]["id"].to_string();
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
                            let handle = value.to_string();
                            if self.state.string("handle") != handle {
                                self.state.set_string("handle".into(), handle.clone());
                                self.state
                                    .mut_dict("app_properties")
                                    .set_string("Agent_Handle".into(), handle);
                                self.save_profile();
                                return true;
                            }
                        } else if !self.state.string("handle").is_empty() {
                            self.state.set_string("handle".into(), "".into());
                            self.state
                                .mut_dict("app_properties")
                                .set_string("Agent_Handle".into(), "".into());
                            self.save_profile();
                            return true;
                        }
                    }

                    Redux::SetFirstName | Redux::GetFirstName => {
                        if !value.is_null() {
                            let first_name = value.to_string();
                            if self.state.string("first_name") != first_name {
                                self.state.set_string("first_name".into(), first_name);
                                self.save_profile();
                                return true;
                            }
                        } else if !self.state.string("first_name").is_empty() {
                            self.state.set_string("first_name".into(), "".into());
                            self.save_profile();
                            return true;
                        }
                    }

                    Redux::SetProfilePic | Redux::GetProfilePic => {
                        if !value.is_null() {
                            let profile_pic = value.to_string();
                            if self.state.string("profile_pic") != profile_pic {
                                self.state.set_string("profile_pic".into(), profile_pic);
                                self.save_profile();
                                return true;
                            }
                        } else if !self.state.string("profile_pic").is_empty() {
                            self.state.set_string("profile_pic".into(), "".into());
                            self.save_profile();
                            return true;
                        }
                    }

                    Redux::Post => {
                        let stamp = index;
                        if !value.is_null() {
                            let address = value.to_string();
                            let post = self.state.mut_dict("posts").mut_dict(&stamp);
                            post.insert("address".into(), address.into());
                        } else {
                            let error = &result["error"];
                            self.state.mut_dict("posts").remove(&stamp);
                            js! { alert(@{format!("Redux::Post error = {}", error.to_string())}) }
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
    fn save_profile(&self) {
        use stdweb::{ web::Storage, unstable::TryInto, };
        let store: Storage = js! { return localStorage }.try_into().unwrap();
        let substate =
            self.state.subset(&["app_properties", "handle", "first_name", "profile_pic"]);
        store.insert("coolcats2_state", &serde_json::to_string(&substate).unwrap()).unwrap();
    }

    fn load_profile(&mut self) {
        use stdweb::{ web::Storage, unstable::TryInto, };
        let store: Storage = js! { return localStorage }.try_into().unwrap();
        if let Some(state) = store.get("coolcats2_state") {
            let substate: State = serde_json::from_str(&state).unwrap();
            self.state.merge(&substate);
        }
    }

    fn coolcats_idx(&mut self, method: &str, params: &[(&str, &str)], redux: &str, index: &str) {
        let call = ToHoloclient::Call(
            (&[self.conductor.as_str(), "coolcats", method][..], params, redux, index).into(),
        );
        self.update(call.into());;
    }

    fn coolcats(&mut self, method: &str, params: &[(&str, &str)], redux: &str) {
        let call = ToHoloclient::Call(
            (&[self.conductor.as_str(), "coolcats", method][..], params, redux).into(),
        );
        self.update(call.into());;
    }

    fn coolcats_np(&mut self, method: &str, redux: &str) {
        let no_params = [("", "")];
        self.coolcats(method, &no_params, redux);
    }

    fn get_my_handle(&mut self) {
        self.coolcats("app_property", &[("key", "Agent_Handle")], Redux::AgentHandle.as_static());
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
            RouterTarget::EditProfile => html! { <EditProfile: counter = counter,/> },
            RouterTarget::Follow => html! { <Follow: counter = counter,/> },
            _ => html! { <App: counter = counter,/> },
        }
    }
}
