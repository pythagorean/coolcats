use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yew::worker::*;

use super::resources::locales::{en, Locale};

pub struct Worker {
    link: AgentLink<Worker>,
    locale: Locale,
}

pub enum Msg {}

#[derive(Serialize, Deserialize)]
pub enum Request {
    LocaleValues(Vec<String>),
}

impl Transferable for Request {}

#[derive(Serialize, Deserialize)]
pub enum Response {
    LocaleValues(HashMap<String, String>),
}

impl Transferable for Response {}

impl Agent for Worker {
    type Reach = Context;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        let locale = en::Locale::initialize();
        Self { link, locale }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::LocaleValues(message_ids) => {
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
        }
    }
}
