use yew::prelude::*;

use crate::holoclient::ToHoloclient;

use super::{
    //utils::Dict,
    state::State,
    components::modal,
    settings::{ self, Settings },
};

const DEFAULT_PROFILE_PIC: &str = "/cat-eating-bird-circle.png";

pub struct App {
    callback: Option<Callback<ToHoloclient>>,
    state: State,
}

pub enum Action {
    //SetDict(String, Dict),
    //SetString(String, String),
    //SetBool(String, bool),
    //SetVec(String, Vec<String>),

    //ResetState,
    //ToggleModal,

    UseHandle(String),
    SetFirstName(String),
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

pub type Params = String;

pub enum ToApp {
    Response(Params),
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub params: Params,
    pub callback: Option<Callback<ToHoloclient>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            params: Params::new(),
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
            state: Default::default(),
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
                //Action::SetDict(   key, value ) => self.state.set_dict(   key, value ),
                //Action::SetString( key, value ) => self.state.set_string( key, value ),
                //Action::SetBool(   key, value ) => self.state.set_bool(   key, value ),
                //Action::SetVec(    key, value ) => self.state.set_vec(    key, value ),

                //Action::ResetState => {
                //    self.state = Default::default();
                //},

                //Action::ToggleModal => {
                //    if let Some(modal_is_open) = self.state.bool("modal_is_open") {
                //        self.state.set_bool("modal_is_open".into(), !modal_is_open);
                //    }
                //}

                Action::UseHandle(handle) => {
                    js! { alert(@{
                        format! { "UseHandle('{}')", handle }
                    })};
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
        let holoclient_response = props.params;
        if !holoclient_response.is_empty() {
            js! { alert(@{
                format! { "App received: {}", holoclient_response }
            })};
        }
        false
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        let app_properties = self.state.dict("app_properties");
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
