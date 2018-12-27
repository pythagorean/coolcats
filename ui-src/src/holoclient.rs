use failure::Error;

use yew::{
    prelude::*,
    format::Json,
    services::websocket::{
        WebSocketService,
        WebSocketTask,
        WebSocketStatus,
    },
};

use crate::app::ToApp;

const HOLOCHAIN_SERVER: &str = "ws://localhost:8888";

pub struct Holoclient {
    ws_service: WebSocketService,
    link: ComponentLink<Holoclient>,
    ws: Option<WebSocketTask>,
    callback: Option<Callback<ToApp>>,
}

#[derive(Serialize, Debug)]
pub struct WsRPC {
    method: String,
    params: Vec<String>,
    timeout: usize,
    ws_opts: String,
}

#[derive(Deserialize, Debug)]
pub struct WsResponse {
    value: String,
}

pub enum WsAction {
    Connect,
    Call(WsRPC),
    Lost,
}

pub enum Msg {
    Callback(ToApp),
    WsAction(WsAction),
    WsReady(Result<WsResponse, Error>),
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

impl From<Call> for WsRPC {
    fn from(call: Call) -> Self {
        WsRPC {
            method: call.method,
            params: call.params,
            timeout: 0,
            ws_opts: String::new(),
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
            ws_service: WebSocketService::new(),
            link,
            ws: None,
            callback: props.callback,
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
                self.update(ToApp::Response(
                    format!{"WsReady: {:?}", response}
                ).into());
            },

            Msg::WsAction(action) => {
                match action {
                    WsAction::Connect => {
                        if self.ws.is_some() { return false; }
                        let callback = self.link.send_back(|Json(data)| Msg::WsReady(data));
                        let notification = self.link.send_back(|status| {
                            match status {
                                WebSocketStatus::Opened => Msg::Ignore,
                                WebSocketStatus::Closed | WebSocketStatus::Error => WsAction::Lost.into(),
                            }
                        });
                        let task = self.ws_service.connect(HOLOCHAIN_SERVER, callback, notification);
                        self.ws = Some(task);
                    },

                    WsAction::Call(rpc) => {
                        js! {
                            alert(@{
                                format! { "WsAction::Call({:?})", rpc }
                            })
                        }
                        self.ws.as_mut().unwrap().send(Json(&rpc));
                    },

                    WsAction::Lost => {
                        self.ws = None;
                    },
                }
            },
        };
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let call = props.params;
        if !call.method.is_empty() {
            let rpc = WsRPC::from(call);
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
