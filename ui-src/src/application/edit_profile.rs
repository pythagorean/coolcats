use yew::prelude::*;

use crate::application::{
    Action,
    context::{ self, ContextAgent },
    state::State,
};

// Declare what state keys will be used by this component
const GETSTATES: [&str; 2] = ["first_name", "profile_pic"];

pub fn getstates() -> Vec<String> {
    GETSTATES.iter().map(|key| key.to_string()).collect()
}

pub struct EditProfile {
    context: Box<Bridge<ContextAgent>>,
    getstate: State,
}

pub enum Msg {
    Action(Action),
    ContextMsg(context::Response),
    GetStates,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub counter: u32,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            counter: 0,
        }
    }
}

impl Component for EditProfile {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let context = ContextAgent::bridge(link.send_back(Msg::ContextMsg));
        let mut component = Self {
            context,
            getstate: State::unset(),
        };
        component.update(Msg::GetStates);
        component
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetStates => {
                self.context.send(context::Request::GetStates(getstates()));
            }

            Msg::Action(msg) => {
                self.context.send(context::Request::Action(msg));
            }

            Msg::ContextMsg(response) => match response {
                context::Response::GetStates(getstate) => {
                    if self.getstate != getstate {
                        self.getstate = getstate;
                        return true;
                    }
                }

                context::Response::Request(_, _) => (),
            },
        };
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        self.update(Msg::GetStates);
        false
    }
}

impl Renderable<EditProfile> for EditProfile {
    fn view(&self) -> Html<Self> {
        if self.getstate.is_empty() {
            return html! { <></> };
        };
        html! {
            <p>{"EditProfile"}</p>
        }
    }
}
