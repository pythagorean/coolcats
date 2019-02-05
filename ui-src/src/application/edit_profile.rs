use yew::prelude::*;

use crate::application::{
    context::{ self, ContextAgent },
    state::State,
    interface::*,
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

impl_interface_component!(EditProfile);

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
