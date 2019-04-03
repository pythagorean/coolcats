use yew::prelude::*;

use crate::application::{
    Action,
    context::{ self, ContextAgent },
    state::State,
};

interface_getstates!("favourites");
interface_component!(Faves, address, String, String::new());
interface_view_only!(Faves);

impl Renderable<Faves> for Faves {
    fn view(&self) -> Html<Self> {
        html! {<></>}
    }
}
