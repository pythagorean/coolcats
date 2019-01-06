use yew::prelude::*;

use crate::app::ToApp;

use super::{
    websocket::{
        WebSocketService,
        WebSocketStatus,
    },
    ws_rpc::{
        self,
        WsRpc,
    },
};

const HOLOCHAIN_SERVER: &str = "ws://localhost:8888";

pub struct Holoclient {
    websocket: Option<WebSocketService>,
    link: ComponentLink<Holoclient>,
    callback: Option<Callback<ToApp>>,
    rpc_id: u32,
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

pub type Params = ws_rpc::Call;

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

            Msg::WsAction(action) => match action {
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
                    self.websocket.as_mut().unwrap().send(&rpc.json());
                },

                WsAction::Lost => {
                    self.websocket = None;
                },
            },
        };
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let call = props.params;
        if call.has_method() {
            self.rpc_id = self.rpc_id + 1;
            let rpc = WsRpc::from((call, self.rpc_id));
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
