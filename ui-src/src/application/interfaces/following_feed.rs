use yew::prelude::*;
use std::collections::HashSet;

use crate::{
    utils::Dict,
    application::{
        Action,
        context::{ self, ContextAgent },
        state::State,
        interfaces::meow::Meow,
    },
};

interface_getstates!("handles", "handle", "follows", "posts");

interface_component!(FollowingFeed);

// This will be mapped to FollowingFeed.local:
pub struct Local {
    post_list: Vec<Dict>,
}

impl Local {
    fn new() -> Self {
        Self {
            post_list: Vec::new(),
        }
    }
}

pub enum LocalMsg {
    NewStates,
}

impl FollowingFeed {
    fn local_update(&mut self, msg: LocalMsg) -> ShouldRender {
        match msg {
            LocalMsg::NewStates => {
                let handles = self.getstate.get_dict("handles");
                let handle = self.getstate.string("handle");
                let follows = self.getstate.get_dict("follows");
                let posts = self.getstate.get_dict("posts");

                let mut follows_plus_self = HashSet::new();
                for key in follows.raw().keys() {
                    follows_plus_self.insert(key.to_string());
                }
                follows_plus_self.insert(handle);

                let mut stamps: Vec<String> = Vec::new();
                for stamp in posts.raw().keys().filter(|stamp| {
                    let post = posts.get_dict(stamp);
                    let author = post.string("author");
                    follows_plus_self.contains(author)
                }) {
                    stamps.push(stamp.to_string());
                }
                stamps.sort_unstable_by(|a, b| {
                    let a: u128 = a.parse().unwrap();
                    let b: u128 = b.parse().unwrap();
                    b.cmp(&a)
                });

                self.local.post_list = Vec::new();
                for stamp in stamps {
                    let mut post = posts.get_dict(&stamp).clone();
                    if post.string("stamp").is_empty() {
                        post.insert("stamp".into(), stamp.into());
                    }

                    let author = post.string("author");
                    let mut user_handle = handles.string(&author);
                    if user_handle.is_empty() {
                        user_handle = author
                    };
                    post.insert("user_handle".into(), user_handle.clone().into());

                    self.local.post_list.push(post);
                }
            }
        }
        true
    }
}

impl Renderable<FollowingFeed> for FollowingFeed {
    fn view(&self) -> Html<Self> {
        let post_list = &self.local.post_list;

        html! {<>
            <div id="meows",>
                { for post_list.iter().map(|post| html! {<Meow: post = post,/>}) }
            </div>
        </>}
    }
}
