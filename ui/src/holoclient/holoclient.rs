use yew::prelude::*;

use serde::Deserialize;

use crate::{application::ToApplication, utils::DictItem};

use super::{
    websocket::{WebSocketService, WebSocketStatus},
    call_rpc::{Call, CallRpc},
};

pub struct Holoclient {
    websocket: Option<WebSocketService>,
    link: ComponentLink<Holoclient>,
    callback: Callback<ToApplication>,
    rpc_id: u32,
}

#[derive(Deserialize, Debug)]
pub struct WsResponse {
    value: String,
}

pub enum WsAction {
    Connect(String),
    Initialize,
    Call(CallRpc),
    Lost,
}

pub enum Msg {
    Callback(ToApplication),
    WsAction(WsAction),
    WsReady(String),
}

impl From<ToApplication> for Msg {
    fn from(msg: ToApplication) -> Self {
        Msg::Callback(msg)
    }
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}

#[derive(PartialEq, Clone)]
pub struct Params {
    rpc: Call,
    redux: String,
    index: String,
}

impl From<(&str, &str)> for Params {
    fn from(args: (&str, &str)) -> Self {
        let rpc: Call = args.0.into();
        let redux = args.1.into();
        Params {
            rpc,
            redux,
            index: String::new(),
        }
    }
}

impl From<(&[&str], &[DictItem], &str)> for Params {
    fn from(args: (&[&str], &[DictItem], &str)) -> Self {
        let rpc: Call = (args.0, args.1).into();
        let redux = args.2.into();
        Params {
            rpc,
            redux,
            index: String::new(),
        }
    }
}

impl From<(&[&str], &[DictItem], &str, &str)> for Params {
    fn from(args: (&[&str], &[DictItem], &str, &str)) -> Self {
        let rpc: Call = (args.0, args.1).into();
        let redux = args.2.into();
        let index = args.3.into();
        Params {
            rpc,
            redux,
            index,
        }
    }
}

impl Default for Params {
    fn default() -> Self {
        Params::new()
    }
}

impl Params {
    pub fn new() -> Self {
        Params {
            rpc: Call::new(),
            redux: String::new(),
            index: String::new(),
        }
    }

    pub fn clear(&mut self) {
        self.rpc.clear();
        self.redux.clear();
        self.index.clear();
    }

    pub fn has_function(&self) -> bool {
        self.rpc.has_function()
    }

    pub fn has_redux(&self) -> bool {
        !self.redux.is_empty()
    }
}

pub enum ToHoloclient {
    Call(Params),
}

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub ws_server: String,
    #[props(required)]
    pub params: Params,
    #[props(required)]
    pub callback: Callback<ToApplication>,
}

impl Component for Holoclient {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut holoclient = Holoclient {
            websocket: None,
            link,
            callback: props.callback,
            rpc_id: 0,
        };
        holoclient.update(WsAction::Connect(props.ws_server).into());
        holoclient
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Callback(msg) => {
                self.callback.emit(msg);
            }

            Msg::WsReady(response) => {
                let response = &json::parse(&response).unwrap();
                if !response["signal"].is_null() {
                    return false;
                }
                let result = response["result"].to_string();
                let rpc_id = response["id"].to_string();
                let mut id_split = rpc_id.split('+');
                let redux = id_split.next().expect("Msg::WsReady: no redux").to_string();
                let index = id_split.next().unwrap_or("").to_string();
                self.update(ToApplication::Redux(result, redux, index).into());
            }

            Msg::WsAction(action) => match action {
                WsAction::Connect(server) => {
                    if self.websocket.is_some() {
                        return false;
                    }
                    let callback = self.link.send_back(Msg::WsReady);
                    let notification = self.link.send_back(|status| match status {
                        WebSocketStatus::Opened => WsAction::Initialize.into(),
                        WebSocketStatus::Closed | WebSocketStatus::Error => WsAction::Lost.into(),
                    });
                    let service = WebSocketService::new(&server, callback, notification);
                    self.websocket = Some(service);
                }

                WsAction::Initialize => {
                    self.update(Msg::Callback(ToApplication::Initialize));
                }

                WsAction::Call(rpc) => {
                    self.websocket.as_mut().unwrap().send(&rpc.json());
                }

                WsAction::Lost => {
                    self.websocket = None;
                }
            },
        };
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let call = props.params;
        if call.has_function() {
            let rpc: CallRpc = if call.has_redux() {
                let rpc_id = format!("{}+{}", call.redux, call.index);
                (call.rpc, rpc_id).into()
            } else {
                self.rpc_id += 1;
                (call.rpc, self.rpc_id).into()
            };
            self.update(WsAction::Call(rpc).into());
        }
        false
    }
}

impl Renderable<Holoclient> for Holoclient {
    fn view(&self) -> Html<Self> {
        html! { <></> }
    }
}
