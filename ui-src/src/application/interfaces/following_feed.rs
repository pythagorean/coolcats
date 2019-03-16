use std::time::Duration;
use std::collections::HashSet;
use yew::{
    prelude::*,
    services::{IntervalService, Task},
};

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
    interval: IntervalService,
    interval_job: Option<Box<Task>>,
}

impl Local {
    fn new() -> Self {
        Self {
            post_list: Vec::new(),
            interval: IntervalService::new(),
            interval_job: None,
        }
    }
}

pub enum LocalMsg {
    NewStates,
    LocalAction(LocalAction),
}

pub enum LocalAction {
    GetPostsBy(Vec<String>),
}

impl From<LocalAction> for Msg {
    fn from(local_action: LocalAction) -> Self {
        LocalMsg::LocalAction(local_action).into()
    }
}

impl FollowingFeed {
    fn local_update(&mut self, local_msg: LocalMsg) -> ShouldRender {
        match local_msg {
            LocalMsg::NewStates => {
                let handles = self.getstate.get_dict("handles");
                let my_handle = self.getstate.string("handle");
                let follows = self.getstate.get_dict("follows");
                let posts = self.getstate.get_dict("posts");

                let mut follows_plus_self = HashSet::new();
                for key in follows.raw().keys() {
                    follows_plus_self.insert(key.to_string());
                }
                follows_plus_self.insert(my_handle.clone());

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

                self.local.post_list = stamps
                    .iter()
                    .map(|stamp| {
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
                        post
                    })
                    .collect();

                if self.local.interval_job.is_none() {
                    let posts_by: Vec<_> = follows_plus_self.into_iter().collect();
                    let send_msg = self
                        .link
                        .send_back(move |_| LocalAction::GetPostsBy(posts_by.clone()).into());
                    let handle = self.local.interval.spawn(Duration::from_secs(2), send_msg);
                    self.local.interval_job = Some(Box::new(handle));
                }
            }

            LocalMsg::LocalAction(local_action) => match local_action {
                LocalAction::GetPostsBy(handles) => {
                    self.get_posts_by(handles);
                }
            },
        }
        true
    }

    fn get_posts_by(&mut self, handles: Vec<String>) {
        self.update(Action::GetPostsBy(handles[0].clone()).into());
    }
}

impl Renderable<FollowingFeed> for FollowingFeed {
    fn view(&self) -> Html<Self> {
        let post_list = &self.local.post_list;

        html! {<>
            <div id="meows",>
                { for post_list.iter().map(|post| {
                    html! { <Meow: post = post,/> }
                })}
            </div>
        </>}
    }
}
