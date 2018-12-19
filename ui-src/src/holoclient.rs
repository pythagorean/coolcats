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
    show: bool,
    ws_service: WebSocketService,
    link: ComponentLink<Holoclient>,
    ws: Option<WebSocketTask>
}

//type AsBinary = bool;

pub enum WsAction {
    Connect,
    //SendData(AsBinary),
    //Disconnect,
    Lost,
}

/// This type is an expected response from a websocket connection.
#[derive(Deserialize, Debug)]
pub struct WsResponse {
    value: u32,
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
    pub show: bool,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            show: false,
        }
    }
}

impl Component for Holoclient {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut holoclient = Holoclient {
            show: props.show,
            ws_service: WebSocketService::new(),
            link,
            ws: None,
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
                    },
                    WsAction::Lost => {
                        self.ws = None;
                    }
                }
            },
            Msg::Ignore => (),
        };
        true
    }
}

impl Renderable<Holoclient> for Holoclient {
    fn view(&self) -> Html<Self> {
        // Render nothing if the "show" prop is false
        if !self.show {
            return html! { <div /> };
        }
        html! {
            <div />
        }
    }
}
