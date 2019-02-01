use yew::prelude::worker::*;

use serde::{Serialize, Deserialize};

use super::state::State;

pub struct ContextAgent {
    link: AgentLink<ContextAgent>,
    root: Option<HandlerId>,
}

pub enum Msg {}

#[derive(Serialize, Deserialize)]
pub enum Request {
    SetRoot,
    GetStates(Vec<String>),
    Response(HandlerId, Box<Response>),
}

impl Transferable for Request {}

impl From<(HandlerId, Response)> for Request {
    fn from(args: (HandlerId, Response)) -> Self {
        let (to_id, response) = args;
        Request::Response(to_id, Box::new(response))
    }
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    GetStates(State),
    Request(HandlerId, Box<Request>),
}

impl Transferable for Response {}

impl From<(HandlerId, Request)> for Response {
    fn from(args: (HandlerId, Request)) -> Self {
        let (from_id, request) = args;
        Response::Request(from_id, Box::new(request))
    }
}

impl ContextAgent {
    fn sendroot(&self, who: HandlerId, request: Request) {
        let root = self.root.unwrap();
        self.link.response(root, (who, request).into());
    }
}

impl Agent for ContextAgent {
    // Available:
    // - `Job` (one per bridge)
    // - `Context` (shared in the same thread)
    // - `Public` (separate thread).
    type Reach = Context; // Spawn only one instance per thread (all components could reach this)
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    // Create an instance with a link to agent's environment.
    fn create(link: AgentLink<Self>) -> Self {
        ContextAgent {
            link,
            root: None,
        }
    }

    // Handle inner messages (of services of `send_back` callbacks)
    fn update(&mut self, msg: Self::Message) { /* ... */ }

    // Handle incoming messages from components of other agents.
    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::SetRoot => self.root = Some(who),
            Request::GetStates(_) => self.sendroot(who, msg),
            Request::Response(who, response) => match *response {
                Response::GetStates(_) => self.link.response(who, *response),
                Response::Request(_, _) => (),
            }
        }
    }
}
