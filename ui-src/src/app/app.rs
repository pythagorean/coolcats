use yew::prelude::*;

use crate::holoclient::ToHoloclient;

use super::{
    utils::Dict,
    components::modal,
    settings::Settings,
};

const DEFAULT_PROFILE_PIC: &str = "/cat-eating-bird-circle.png";

pub enum State {
    Initialize,
}

pub struct App {
    callback: Option<Callback<ToHoloclient>>,
    state: Dict,
}

pub enum Msg {
    Callback(ToHoloclient),
    State(State),
}

impl From<ToHoloclient> for Msg {
    fn from(msg: ToHoloclient) -> Self {
        Msg::Callback(msg)
    }
}

impl From<State> for Msg {
    fn from(action: State) -> Self {
        Msg::State(action)
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
        let mut app = App {
            callback: props.callback,
            state: Dict::new(),
        };
        app.update(State::Initialize.into());
        app
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Callback(msg) => {
                if let Some(ref mut callback) = self.callback {
                    callback.emit(msg);
                }
            },

            Msg::State(action) => match action {
                State::Initialize => {
                    self.state.clear();
                    self.state.insert("posts".into(), Dict::new().into());
                    self.state.insert("modal_is_open".into(), true.into());
                    self.state.insert("profile_pic".into(), "".into());
                },
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
        let _posts = self.state.dict("posts".into());
        let modal_is_open = self.state.bool("modal_is_open".into());
        let profile_pic = self.state.string("profile_pic".into());

        match modal_is_open {
            true => html! {
                <div style={ modal::BACKDROP_STYLE },>
                    <div style={ modal::MODAL_STYLE },>
                        <div align="center",>
                            <p classname="h1",>{ "Welcome to Coolcats2!" }</p>
                        </div>
                        <Settings:/>
                    </div>
                </div>
            }, _ => html! {
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
