use yew::prelude::*;

use crate::application::{
    context::{ self, ContextAgent },
    state::State,
    interface::*,
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
