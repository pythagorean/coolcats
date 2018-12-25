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

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Debug)]
pub struct WsRequest {
    value: String,
}

/// This type is an expected response from a websocket connection.
#[derive(Deserialize, Debug)]
pub struct WsResponse {
    value: String,
}

pub enum WsAction {
    Connect,
    SendData(WsRequest),
    //Disconnect,
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

pub type Params = Vec<String>;

pub enum ToHoloclient {
    Msg(Params),
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
            Msg::Callback(msg) => {
                if let Some(ref mut callback) = self.callback {
                    callback.emit(msg);
                }
            },

            Msg::WsReady(response) => {
                self.update(ToApp::Msg(
                    format!{"WsReady: {:?}", response}
                ).into());
            },

            Msg::WsAction(action) => {
                match action {
                    WsAction::Connect => {
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

                    WsAction::SendData(request) => {
                        self.ws.as_mut().unwrap().send(Json(&request));
                    }

                    WsAction::Lost => {
                        self.ws = None;
                    }
                }
            },

            Msg::Ignore => (),
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if !props.params.is_empty() {
            let value = props.params.join("/");
            let request = WsRequest { value };
            self.update(WsAction::SendData(request).into());
        }
        false
    }
}

impl Renderable<Holoclient> for Holoclient {
    fn view(&self) -> Html<Self> {
        html! { <div /> }
    }
}
