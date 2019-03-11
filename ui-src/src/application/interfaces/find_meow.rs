use yew::prelude::*;

use crate::application::{
    context::{ self, ContextAgent },
    state::State,
    interfaces::meow::Meow,
};

interface_getstates!("handles", "posts");

pub struct FindMeow {
    context: Box<Bridge<ContextAgent>>,
    getstate: State,
    address: String,
}

pub enum Msg {
    ContextMsg(context::Response),
    GetStates,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub address: String,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            address: String::new(),
        }
    }
}

impl Component for FindMeow {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let context = ContextAgent::bridge(link.send_back(Msg::ContextMsg));
        let mut component = Self {
            context,
            getstate: State::unset(),
            address: props.address,
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
                    if self.getstate != getstate {
                        self.getstate = getstate;
                        return true;
                    }
                }

                context::Response::Request(_, _) => (),
            },
        };
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.address = props.address;
        true
    }
}

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
