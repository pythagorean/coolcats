use std::time::Duration;
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

interface_getstates!("connected", "handles", "handle", "follows", "posts");
interface_component!(FollowingFeed);

// This will be mapped to FollowingFeed.local:
pub struct Local {
    my_handle: String,
    posts_by_len: usize,
    post_list: Vec<Dict>,
    interval: IntervalService,
    interval_job: Option<Box<Task>>,
}

impl Local {
    fn new() -> Self {
        Self {
            my_handle: "".into(),
            posts_by_len: 0,
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
                let connected = self.getstate.bool("connected").unwrap();
                let my_handle = self.getstate.string("handle").clone();
                if connected && self.local.my_handle != my_handle {
                    self.local.my_handle = my_handle.clone();
                    self.get_following();
                }

                let follows = self.getstate.get_dict("follows");
                let mut posts_by: Vec<_> = follows.raw().keys().cloned().collect();
                if !posts_by.contains(&my_handle) {
                    posts_by.push(my_handle);
                }

                self.get_feed(posts_by.as_slice());

                if connected && self.local.posts_by_len != posts_by.len() {
                    self.local.posts_by_len = posts_by.len();
                    if let Some(mut task) = self.local.interval_job.take() {
                        task.cancel()
                    }
                    self.get_posts_by(posts_by.clone());
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
        self.update(Action::GetPostsBy(handles).into());
    }

    fn get_following(&mut self) {
        self.update(Action::GetFollowing(self.local.my_handle.clone()).into());
    }

    fn get_feed(&mut self, posts_by: &[String]) {
        let handles = self.getstate.get_dict("handles");
        let posts = self.getstate.get_dict("posts");

        let mut stamps: Vec<String> = Vec::new();
        for stamp in posts.raw().keys().filter(|stamp| {
            let post = posts.get_dict(stamp);
            let author = post.string("author");
            posts_by.contains(author)
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

                let author = post.string("author").clone();
                let mut user_handle = handles.string(&author).clone();
                if user_handle.is_empty() {
                    user_handle = author
                };
                post.insert("user_handle".into(), user_handle.into());
                post
            })
            .collect();
    }
}

impl Renderable<FollowingFeed> for FollowingFeed {
    fn view(&self) -> Html<Self> {
        let post_list = &self.local.post_list;

        html! {<>
            <div id="meows",>
                { for post_list.iter().map(|post| {
                    html! { <Meow: counter = self.counter, post = post,/> }
                })}
            </div>
        </>}
    }
}
