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
    state: State,
}

pub enum Action {
    ResetState,
    SetString(String, String),
}

pub enum Msg {
    Callback(ToHoloclient),
    Action(Action),
    FromComponent(Action),
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
            },

            Msg::Action(action) => match action {
                Action::ResetState => {
                    self.state = Default::default();
                },

                Action::SetString(key, value) => {
                    self.state.set_string(key, value);
                },
            },

            Msg::FromComponent(action) => {
                self.update(action.into());
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
        let modal_is_open = self.state.bool("modal_is_open");
        let profile_pic = self.state.string("profile_pic");

        match modal_is_open {
            Some(true) => html! {
                <div style={ modal::BACKDROP_STYLE },>
                    <div style={ modal::MODAL_STYLE },>
                        <div align="center",>
                            <p classname="h1",>{ "Welcome to Coolcats2!" }</p>
                        </div>
                        <Settings:
                            substate = self.state.subset(settings::USES_STATE.to_vec()),
                            callback = |action| Msg::FromComponent(action),
                        />
                    </div>
                </div>
            },
            Some(false) => html! {
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
            },
            _ => html! { <></> },
        }
    }
}
