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

interface_getstates!("handles", "posts");

pub struct UserFeed {
    context: Box<Bridge<ContextAgent>>,
    getstate: State,
    handle: String,
    post_list: Vec<Dict>,
    link: ComponentLink<UserFeed>,
    interval: IntervalService,
    interval_job: Option<Box<Task>>,
}

pub enum Msg {
    Action(Action),
    ContextMsg(context::Response),
    GetStates,
    LocalAction(LocalAction),
}

pub enum LocalAction {
    GetReady,
    GetPosts,
}

impl From<Action> for Msg {
    fn from(action: Action) -> Self {
        Msg::Action(action)
    }
}

impl From<LocalAction> for Msg {
    fn from(local_action: LocalAction) -> Self {
        Msg::LocalAction(local_action)
    }
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub handle: String,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            handle: String::new(),
        }
    }
}

impl Component for UserFeed {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let context = ContextAgent::bridge(link.send_back(Msg::ContextMsg));
        let mut component = Self {
            context,
            getstate: State::unset(),
            handle: props.handle,
            post_list: Vec::new(),
            link,
            interval: IntervalService::new(),
            interval_job: None,
        };
        component.update(Msg::GetStates);
        component
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetStates => {
                self.context.send(context::Request::GetStates(getstates()));
            }

            Msg::Action(action) => {
                self.context.send(context::Request::Action(action));
            }

            Msg::ContextMsg(response) => match response {
                context::Response::GetStates(getstate) => {
                    if self.getstate.is_empty() && !getstate.is_empty() {
                        self.getstate = getstate;
                        self.update(LocalAction::GetReady.into());
                        return true;
                    }
                    self.getstate = getstate;
                    return self.get_feed();
                }

                context::Response::Request(_, _) => (),
            },

            Msg::LocalAction(local_action) => match local_action {
                LocalAction::GetReady => {
                    self.get_feed();
                    self.get_posts();
                    if self.interval_job.is_none() {
                        let send_msg = self.link.send_back(|_| LocalAction::GetPosts.into());
                        let handle = self.interval.spawn(Duration::from_secs(1), send_msg);
                        self.interval_job = Some(Box::new(handle));
                    }
                }

                LocalAction::GetPosts => {
                    self.get_posts();
                }
            },
        };
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.handle = props.handle;
        self.update(Msg::GetStates);
        true
    }
}

impl UserFeed {
    fn get_posts(&mut self) {
        self.update(Action::GetPostsBy(vec![self.handle.clone()]).into());
    }

    fn get_feed(&mut self) -> bool {
        let posts = self.getstate.get_dict("posts");
        let handles = self.getstate.get_dict("handles");
        let handle = &self.handle;

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

                let author = post.string("author");
                let mut user_handle = handles.string(author);
                if user_handle.is_empty() {
                    user_handle = author
                };
                post.insert("user_handle".into(), user_handle.clone().into());
                post
            })
            .collect();

        if self.post_list != post_list {
            self.post_list = post_list;
            return true;
        }
        false
    }
}

impl Renderable<UserFeed> for UserFeed {
    fn view(&self) -> Html<Self> {
        let handle = &self.handle;
        let post_list = &self.post_list;

        html! {<>
            <div id="meows",>
                <h2 id="user-header",>
                    {handle}
                </h2>
                { for post_list.iter().map(|post| {
                    html! { <Meow: post = post,/> }
                })}
            </div>
        </>}
    }
}
