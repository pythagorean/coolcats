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

const MAX_HANDLE_LENGTH: usize = 20;

pub struct Settings {
    show: bool,
    use_handle_text: String,
    ws_service: WebSocketService,
    link: ComponentLink<Settings>,
    ws: Option<WebSocketTask>
}

fn get_first_name() -> Option<String> { Some("".into()) }

fn set_first_name(name: &str) {
    js! { alert("set_first_name: " + @{name}) }
}

fn toggle_modal() {}

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
    UpdateHandleText(ChangeData),
    OnHandleSubmit,
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

impl Component for Settings {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut settings = Settings {
            show: props.show,
            use_handle_text: String::from(""),
            ws_service: WebSocketService::new(),
            link,
            ws: None,
        };
        settings.update(WsAction::Connect.into());
        settings
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateHandleText(ChangeData::Value(handle_text)) => {
                self.use_handle_text = handle_text;
                return false;
            },
            Msg::OnHandleSubmit => {
                js! {
                    assert.strictEqual(1,2);
                }
                self.on_handle_submit();
            },
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
                        let task = self.ws_service.connect("ws://localhost:8888", callback, notification);
                        self.ws = Some(task);
                    },
                    WsAction::Lost => {
                        self.ws = None;
                    }
                }
            },
            Msg::Ignore => (),
            _ => ()
        };
        true
    }
}

impl Settings {
    fn use_handle(&mut self, _handle: &str) {
    }

    fn on_handle_submit(&mut self) {
        // empty string given as input
        if self.use_handle_text.len() == 0 { return };

        // max characters exceeded
        if self.use_handle_text.len() > MAX_HANDLE_LENGTH {
            self.use_handle_text = String::from("");
            return
        }

        self.use_handle("abc");

        //self.use_handle(self.use_handle_text);

        // check if a name has been set, and if not default to handle
        match get_first_name() {
            Some(ref first_name) if first_name.len() > 1 => (),
            _ => set_first_name(&self.use_handle_text),
        };

        toggle_modal();
    }
}

impl Renderable<Settings> for Settings {
    fn view(&self) -> Html<Self> {
        // Render nothing if the "show" prop is false
        if !self.show {
            return html! { </> };
        }
        let use_handle_text = &self.use_handle_text;
        let handle_taken = false;
        html! {
            <div classname="panel panel-default",>
                <div classname="panel-body",>
                    <div style="padding-left: 30; padding-bottom: 10;",>
                        <p
                            classname="text-info",
                            style={
                                if use_handle_text.len() == 0 && handle_taken == false {
                                    "display: inline;"
                                } else {
                                    "display: none;"
                                }
                            },
                        >
                            {"Set your handle to get meowing"}
                        </p>
                    </div>
                    <div style="padding-left: 30; padding-bottom: 10;",>
                        <p
                            classname="text-danger",
                            style={
                                if handle_taken == true {
                                    "display: inline;"
                                } else {
                                    "display: none;"
                                }
                            },
                        >
                            {"This handle already has a home, try something else!"}
                        </p>
                    </div>
                    <div classname="col-xs-8",>
                        <div classname="form-group input-icon",>
                            <i>{"@"}</i>
                            <input
                                value=use_handle_text,
                                onchange=|input| Msg::UpdateHandleText(input),
                                onkeypress=|pressed| {
                                    if pressed.key() == "Enter" { Msg::OnHandleSubmit }
                                    else { Msg::Ignore }
                                },
                                type="text",
                                classname="form-control",
                                id="myHandle",
                                placeholder="handle",
                            />
                        </div>
                    </div>
                    <div classname="col-xs-2",>
                        <button
                            id="setHandleButton",
                            classname="btn btn-primary",
                            onclick=|_| Msg::OnHandleSubmit,
                        >
                            {"Set Handle"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
