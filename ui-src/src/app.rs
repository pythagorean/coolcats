use yew::prelude::*;

use crate::components::modal::{BACKDROP_STYLE, MODAL_STYLE};
use crate::settings::Settings;

const DEFAULT_PROFILE_PIC: &str = "/cat-eating-bird-circle.png";

pub struct App {
    show: bool,
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub show: bool,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            show: false,
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            show: props.show,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        let modal_is_open = true; // self.modal_is_open;
        let profile_pic = "";
        match modal_is_open {
            true => html! {
                <div style={BACKDROP_STYLE},>
                    <div style={MODAL_STYLE},>
                        <div align="center",>
                            <p classname="h1",>{"Welcome to Coolcats2!"}</p>
                        </div>
                        <Settings: show=true,/>
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
                                    {&format!("show: {}", self.show)}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
    }
}
