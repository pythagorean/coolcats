use yew::prelude::*;

use crate::application::{
    Action,
    context::{ self, ContextAgent },
    state::State,
};

// Declare what state keys will be used by this component
const GETSTATES: [&str; 1] = ["follows"];

pub fn getstates() -> Vec<String> {
    GETSTATES.iter().map(|key| key.to_string()).collect()
}

pub struct Follow {
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

impl_interface_component!(Follow);

impl Renderable<Follow> for Follow {
    fn view(&self) -> Html<Self> {
        if self.getstate.is_empty() {
            return html! { <></> };
        };
        html! {
            <p>{"Follow"}</p>
        }
    }
}
