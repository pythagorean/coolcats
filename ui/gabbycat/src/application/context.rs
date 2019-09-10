use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yew::worker::*;

use super::{
    resources::locales::{en, Locale},
    state::State,
};

pub struct Worker {
    link: AgentLink<Worker>,
    locale: Locale,
    state: State,
}

pub enum Msg {}

#[derive(Serialize, Deserialize)]
pub enum Request {
    GetLocaleValues(Vec<String>),
    GetStateValues(Vec<String>),
}

impl Transferable for Request {}

#[derive(Serialize, Deserialize)]
pub enum Response {
    LocaleValues(HashMap<String, String>),
    StateValues(State),
}

impl Transferable for Response {}

impl Agent for Worker {
    type Reach = Context;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            locale: en::Locale::initialize(),
            state: State::initialize(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::GetLocaleValues(message_ids) => {
                let values: HashMap<String, String> = message_ids
                    .iter()
                    .map(|message_id| {
                        (
                            message_id.into(),
                            self.locale.get_value(message_id).to_string(),
                        )
                    })
                    .collect();
                self.link.response(who, Response::LocaleValues(values));
            }
            Request::GetStateValues(keys) => {
                let keys: Vec<_> = keys.iter().map(String::as_str).collect();
                let values = self.state.substate(keys.as_slice());
                self.link.response(who, Response::StateValues(values));
            }
        }
    }
}
