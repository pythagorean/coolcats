use yew::prelude::*;

use crate::components::modal::{BACKDROP_STYLE, MODAL_STYLE};
use crate::settings::Settings;
use crate::ToHoloclient;

const DEFAULT_PROFILE_PIC: &str = "/cat-eating-bird-circle.png";

pub struct App {
    root_callback: Option<Callback<ToHoloclient>>,
}

pub enum Msg {
    ToRoot(ToHoloclient),
}

impl From<ToHoloclient> for Msg {
    fn from(msg: ToHoloclient) -> Self {
        Msg::ToRoot(msg)
    }
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub params: String,
    pub root_callback: Option<Callback<ToHoloclient>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            params: "".into(),
            root_callback: None,
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            root_callback: props.root_callback,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToRoot(msg) => {
                if let Some(ref mut root_callback) = self.root_callback {
                    root_callback.emit(msg);
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        //self.to_model = props.to_model;
        false
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        return html! {
            <div>
                <button
                    onclick=|_| ToHoloclient::Msg("Test".into()).into(),
                >
                    { "Test" }
                </button>
            </div>
        };

        let modal_is_open = true; // self.modal_is_open;
        let profile_pic = "";
        match modal_is_open {
            true => html! {
                <div style={BACKDROP_STYLE},>
                    <div style={MODAL_STYLE},>
                        <div align="center",>
                            <p classname="h1",>{"Welcome to Coolcats2!"}</p>
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
                                            if !profile_pic.is_empty() {profile_pic}
                                            else {DEFAULT_PROFILE_PIC}
                                        },
                                        alt="user-profile",
                                    />
                                </div>
                                <div id="displayName",>
                                    {&format!("show: {}", true)}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
    }
}
