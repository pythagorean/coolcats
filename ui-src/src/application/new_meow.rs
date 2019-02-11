use yew::prelude::*;

use crate::application::{
    Action,
    context::{ self, ContextAgent },
    state::State,
};

// Declare what state keys will be used by this component
const GETSTATES: [&str; 1] = ["handle"];

pub fn getstates() -> Vec<String> {
    lazy_static! {
        static ref VS: Vec<String> = GETSTATES.iter().map(|key| key.to_string()).collect();
    }
    VS.to_vec()
}

interface_view_only!(NewMeow);
interface_component!(NewMeow);

impl Renderable<NewMeow> for NewMeow {
    fn view(&self) -> Html<Self> {
        if self.getstate.is_empty() {
            return html! {<></>};
        };
        html! {<>
            <h1>{"NewMeow"}</h1>
        </>}
    }
}
