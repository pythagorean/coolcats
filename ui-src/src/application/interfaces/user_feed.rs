use std::time::Duration;
use yew::{
    prelude::*,
    services::{IntervalService, Task},
};

use crate::{
    utils::Dict,
    application::{
        Action,
        context::{self, ContextAgent},
        state::State,
        interfaces::meow::Meow,
    },
};

interface_getstates!("handles", "posts");
interface_component!(UserFeed, params, (u32, String), (0, String::new()));

// This will be mapped to UserFeed.local:
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
    GetReady,
    GetPosts,
}

impl From<LocalAction> for Msg {
    fn from(local_action: LocalAction) -> Self {
        LocalMsg::LocalAction(local_action).into()
    }
}

impl UserFeed {
    fn local_update(&mut self, local_msg: LocalMsg) -> ShouldRender {
        match local_msg {
            LocalMsg::NewStates => {
                if self.local.interval_job.is_none() {
                    self.update(LocalAction::GetReady.into());
                    return true;
                }
                return self.get_feed();
            }

            LocalMsg::LocalAction(local_action) => match local_action {
                LocalAction::GetReady => {
                    self.get_feed();
                    self.get_posts();
                    let send_msg = self.link.send_back(|_| LocalAction::GetPosts.into());
                    let handle = self.local.interval.spawn(Duration::from_secs(1), send_msg);
                    self.local.interval_job = Some(Box::new(handle));
                }

                LocalAction::GetPosts => {
                    self.get_posts();
                }
            },
        }
        false
    }

    fn get_posts(&mut self) {
        let (_, handle) = &self.params;
        let handle = handle.clone();
        self.update(Action::GetPostsBy([handle].to_vec()).into());
    }

    fn get_feed(&mut self) -> bool {
        let posts = self.getstate.get_dict("posts");
        let handles = self.getstate.get_dict("handles");
        let (_, handle) = &self.params;

        let mut stamps: Vec<String> = Vec::new();
        for stamp in posts.raw().keys().filter(|stamp| {
            let post = posts.get_dict(stamp);
            let author = post.string("author");
            author == handle
        }) {
            stamps.push(stamp.to_string());
        }
        stamps.sort_unstable_by(|a, b| {
            let a: u128 = a.parse().unwrap();
            let b: u128 = b.parse().unwrap();
            b.cmp(&a)
        });

        let post_list = stamps
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

        if self.local.post_list != post_list {
            self.local.post_list = post_list;
            return true;
        }
        false
    }
}

impl Renderable<UserFeed> for UserFeed {
    fn view(&self) -> Html<Self> {
        let (counter, handle) = &self.params;
        let post_list = &self.local.post_list;

        html! {<>
            <div id="meows",>
                <h2 id="user-header",>
                    {handle}
                </h2>
                { for post_list.iter().map(|post| {
                    html! { <Meow: counter = counter, post = post,/> }
                })}
            </div>
        </>}
    }
}
