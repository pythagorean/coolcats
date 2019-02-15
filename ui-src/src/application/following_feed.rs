use yew::prelude::*;

use crate::application::{
    Action,
    context::{ self, ContextAgent },
    state::State,
};

pub fn getstates() -> Vec<String> {
    Vec::new()
}

interface_view_only!(FollowingFeed);
interface_component!(FollowingFeed);

impl Renderable<FollowingFeed> for FollowingFeed {
    fn view(&self) -> Html<Self> {
        html! {<>{"FollowingFeed"}</>}
    }
}
