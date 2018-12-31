use yew::prelude::*;

use crate::{
    components::modal,
    settings::Settings,
    holoclient::ToHoloclient,
};

const DEFAULT_PROFILE_PIC: &str = "/cat-eating-bird-circle.png";

pub struct App {
    callback: Option<Callback<ToHoloclient>>,
}

pub enum Msg {
    Callback(ToHoloclient),
}

impl From<ToHoloclient> for Msg {
    fn from(msg: ToHoloclient) -> Self {
        Msg::Callback(msg)
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Callback(msg) => {
                if let Some(ref mut callback) = self.callback {
                    callback.emit(msg);
                }
            }
        }
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
        return html! {
            <div>
                <button onclick=|_| ToHoloclient::Call("info/instances".into()).into(),>
                    { "Test" }
                </button>
            </div>
        };

        let modal_is_open = true; // self.modal_is_open;
        let profile_pic = "";
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
                                            if !profile_pic.is_empty() { profile_pic }
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
