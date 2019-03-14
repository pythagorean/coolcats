use yew::prelude::*;

use crate::{
    utils::Dict,
    application::{
        context::{ self, ContextAgent },
        state::State,
        interfaces::meow::Meow,
    },
};

interface_getstates!("handles", "posts");

pub struct HashtagFeed {
    context: Box<Bridge<ContextAgent>>,
    getstate: State,
    hashtag: String,
    post_list: Vec<Dict>,
}

pub enum Msg {
    ContextMsg(context::Response),
    GetStates,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub hashtag: String,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            hashtag: String::new(),
        }
    }
}

impl Component for HashtagFeed {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let context = ContextAgent::bridge(link.send_back(Msg::ContextMsg));
        let mut component = Self {
            context,
            getstate: State::unset(),
            hashtag: props.hashtag,
            post_list: Vec::new(),
        };
        component.update(Msg::GetStates);
        component
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetStates => {
                self.context.send(context::Request::GetStates(getstates()));
            }

            Msg::ContextMsg(response) => match response {
                context::Response::GetStates(getstate) => {
                    self.getstate = getstate;
                    return self.get_feed();
                }

                context::Response::Request(_, _) => (),
            },
        };
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.hashtag = props.hashtag;
        self.update(Msg::GetStates);
        true
    }
}

impl HashtagFeed {
    fn get_feed(&mut self) -> bool {
        let posts = self.getstate.get_dict("posts");
        let handles = self.getstate.get_dict("handles");
        let hashedtag = "#".to_owned() + &self.hashtag;

        let mut stamps: Vec<String> = Vec::new();
        for stamp in posts.raw().keys().filter(|stamp| {
            let post = posts.get_dict(stamp);
            let message = post.string("message");
            message.contains(&hashedtag)
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

impl Renderable<HashtagFeed> for HashtagFeed {
    fn view(&self) -> Html<Self> {
        let hashtag = &self.hashtag;
        let post_list = &self.post_list;

        html! {<>
            <div id="meows",>
                <h2 id="user-header",>
                    {"#"}{hashtag}
                </h2>
                { for post_list.iter().map(|post| {
                    html! { <Meow: post = post,/> }
                })}
            </div>
        </>}
    }
}
