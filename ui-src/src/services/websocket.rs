use yew::{
    services::Task,
    callback::Callback,
};

use stdweb::{
    traits::IMessageEvent,
    web::{
        WebSocket,
        SocketReadyState,
        IEventTarget,
        event::{
            SocketOpenEvent,
            SocketCloseEvent,
            SocketErrorEvent,
            SocketMessageEvent,
        },
    },
};

pub enum WebSocketStatus {
    Opened,
    Closed,
    Error,
}

pub struct WebSocketService {
    ws: WebSocket,
    notification: Callback<WebSocketStatus>,
}

impl WebSocketService {
    pub fn new(
        server: &str,
        callback: Callback<String>,
        notification: Callback<WebSocketStatus>
    ) -> Self {
        let ws = WebSocket::new(server).expect("Unable to connect to websocket");

        let n = notification.clone();
        ws.add_event_listener(move |_: SocketOpenEvent| {
            n.emit(WebSocketStatus::Opened);
        });

        let n = notification.clone();
        ws.add_event_listener(move |_: SocketCloseEvent| {
            n.emit(WebSocketStatus::Closed);
        });

        let n = notification.clone();
        ws.add_event_listener(move |_: SocketErrorEvent| {
            n.emit(WebSocketStatus::Error);
        });

        ws.add_event_listener(move |event: SocketMessageEvent| {
            if let Some(data) = event.data().into_text() {
                callback.emit(data.into());
            }
        });

        Self { ws, notification }
    }

    pub fn send(&mut self, json: &str) {
        if self.ws.send_text(json).is_err() {
            self.notification.emit(WebSocketStatus::Error);
        } else {
            js! { alert(@{
                format! { "Sent: {}", json }
            })};
        }
    }
}

impl Task for WebSocketService {
    fn is_active(&self) -> bool {
        self.ws.ready_state() == SocketReadyState::Open
    }

    fn cancel(&mut self) {
        self.ws.close();
    }
}

impl Drop for WebSocketService {
    fn drop(&mut self) {
        if self.is_active() {
            self.cancel();
        }
    }
}
