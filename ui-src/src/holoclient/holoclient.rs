use yew::prelude::*;

use crate::{
    holoclient::websocket::{
        WebSocketService,
        WebSocketStatus,
    },
    app::ToApp,
};

const HOLOCHAIN_SERVER: &str = "ws://localhost:8888";

pub struct Holoclient {
    websocket: Option<WebSocketService>,
    link: ComponentLink<Holoclient>,
    callback: Option<Callback<ToApp>>,
    rpc_id: u32,
}

#[derive(Serialize, Debug)]
pub struct WsRpc {
    jsonrpc: String,
    method: String,
    params: Vec<String>,
    id: u32,
}

#[derive(Serialize, Debug)]
pub struct WsRpcNoParams {
    jsonrpc: String,
    method: String,
    params: Option<String>,
    id: u32,
}

#[derive(Deserialize, Debug)]
pub struct WsResponse {
    value: String,
}

pub enum WsAction {
    Connect,
    Call(WsRpc),
    Lost,
}

pub enum Msg {
    Callback(ToApp),
    WsAction(WsAction),
    WsReady(String),
    Ignore,
}

impl From<ToApp> for Msg {
    fn from(msg: ToApp) -> Self {
        Msg::Callback(msg)
    }
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Call {
    method: String,
    params: Vec<String>,
}

impl Call {
    pub fn new() -> Self {
        Call {
            method: String::new(),
            params: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.method.clear();
        self.params.clear();
    }
}

impl From<&str> for Call {
    fn from(method: &str) -> Self {
        Call {
            method: method.into(),
            params: Vec::new(),
        }
    }
}

impl From<(&str, Vec<String>)> for Call {
    fn from(args: (&str, Vec<String>)) -> Self {
        Call {
            method: args.0.into(),
            params: args.1,
        }
    }
}

impl From<Vec<String>> for Call {
    fn from(method: Vec<String>) -> Self {
        Call {
            method: method.join("/"),
            params: Vec::new(),
        }
    }
}

impl From<(Vec<String>, Vec<String>)> for Call {
    fn from(vecs: (Vec<String>, Vec<String>)) -> Self {
        Call {
            method: vecs.0.join("/"),
            params: vecs.1,
        }
    }
}

impl From<(Call, u32)> for WsRpc {
    fn from(call_id: (Call, u32)) -> Self {
        WsRpc {
            jsonrpc: "2.0".into(),
            method: call_id.0.method,
            params: call_id.0.params,
            id: call_id.1,
        }
    }
}

impl From<WsRpc> for WsRpcNoParams {
    fn from(rpc: WsRpc) -> Self {
        WsRpcNoParams {
            jsonrpc: rpc.jsonrpc,
            method: rpc.method,
            params: None,
            id: rpc.id,
        }
    }
}

pub type Params = Call;

pub enum ToHoloclient {
    Call(Params),
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub params: Params,
    pub callback: Option<Callback<ToApp>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            params: Params::new(),
            callback: None,
        }
    }
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
        holoclient.update(WsAction::Connect.into());
        holoclient
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => (),

            Msg::Callback(msg) => {
                if let Some(ref mut callback) = self.callback {
                    callback.emit(msg);
                }
            },

            Msg::WsReady(response) => {
                self.update(ToApp::Response(response).into());
            },

            Msg::WsAction(action) => {
                match action {
                    WsAction::Connect => {
                        if self.websocket.is_some() { return false; }
                        let callback = self.link.send_back(|data| Msg::WsReady(data));
                        let notification = self.link.send_back(|status| {
                            match status {
                                WebSocketStatus::Opened => Msg::Ignore,
                                WebSocketStatus::Closed | WebSocketStatus::Error => WsAction::Lost.into(),
                            }
                        });
                        let service = WebSocketService::new(HOLOCHAIN_SERVER, callback, notification);
                        self.websocket = Some(service);
                    },

                    WsAction::Call(rpc) => {
                        let json: String;
                        if rpc.params.is_empty() {
                            let rpc = WsRpcNoParams::from(rpc);
                            json = serde_json::to_string(&rpc).unwrap();
                        } else {
                            json = serde_json::to_string(&rpc).unwrap();
                        }
                        self.websocket.as_mut().unwrap().send(&json);
                    },

                    WsAction::Lost => {
                        self.websocket = None;
                    },
                }
            },
        };
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let call = props.params;
        if !call.method.is_empty() {
            self.rpc_id = self.rpc_id + 1;
            let rpc = WsRpc::from((call, self.rpc_id));
            self.update(WsAction::Call(rpc).into());
        }
        false
    }
}

impl Renderable<Holoclient> for Holoclient {
    fn view(&self) -> Html<Self> {
        html! { <div /> }
    }
}
