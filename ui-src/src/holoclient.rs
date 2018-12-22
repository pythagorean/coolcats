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

const HOLOCHAIN_SERVER: &str = "ws://localhost:8888";

pub struct Holoclient {
    ws_service: WebSocketService,
    link: ComponentLink<Holoclient>,
    ws: Option<WebSocketTask>,
    to_model: Option<Callback<String>>,
}

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Debug)]
pub struct WsRequest {
    value: u32,
}

/// This type is an expected response from a websocket connection.
#[derive(Deserialize, Debug)]
pub struct WsResponse {
    value: u32,
}

pub enum WsAction {
    Connect,
    SendData(WsRequest),
    //Disconnect,
    Lost,
}

pub enum Msg {
    WsAction(WsAction),
    WsReady(Result<WsResponse, Error>),
    Ignore,
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub to_model: Option<Callback<String>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            to_model: None,
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
            to_model: props.to_model,
        };
        holoclient.update(WsAction::Connect.into());
        holoclient
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::WsReady(_response) => {
                js! { alert("WsReady") }
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
                        //let request = WsRequest { value: 420 };
                        //self.update(WsAction::SendData(request).into());
                        if let Some(ref mut to_model) = self.to_model {
                            to_model.emit("We Get Signal!".into());
                        }
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        //self.to_model = props.to_model;
        false
    }
}

impl Renderable<Holoclient> for Holoclient {
    fn view(&self) -> Html<Self> {
        html! { <div /> }
    }
}
