use yew::prelude::*;

use crate::application::{
    Action,
    context::{ self, ContextAgent },
    state::State,
    interfaces::meow::Meow,
};

interface_getstates!("handles", "posts");
interface_component!(FindMeow, address, String, String::new());
interface_view_only!(FindMeow);

impl Renderable<FindMeow> for FindMeow {
    fn view(&self) -> Html<Self> {
        if self.getstate.is_empty() {
            return html! { <></> };
        };
        let handles = self.getstate.get_dict("handles");
        let posts = self.getstate.get_dict("posts");
        let address = &self.address;

        if let Some(stamp) = posts.raw().keys().find(|stamp| {
            let post = posts.get_dict(stamp);
            post.string("address") == address
        }) {
            let mut post = posts.get_dict(stamp).clone();
            if post.string("stamp").is_empty() {
                post.insert("stamp".into(), stamp.clone().into());
            }
            let author = post.string("author");
            let mut user_handle = handles.string(author);
            if user_handle.is_empty() {
                user_handle = author
            };
            post.insert("user_handle".into(), user_handle.clone().into());
            return html! { <Meow: post = post,/> };
        }
        html! { <></> }
    }
}
