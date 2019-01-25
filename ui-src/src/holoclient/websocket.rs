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
    websocket: WebSocket,
    notification: Callback<WebSocketStatus>,
}

impl WebSocketService {
    pub fn new(
        server: &str,
        callback: Callback<String>,
        notification: Callback<WebSocketStatus>
    ) -> Self {
        let websocket = WebSocket::new(server).expect("Unable to connect to websocket");

        let notify = notification.clone();
        websocket.add_event_listener(move |_: SocketOpenEvent| {
            notify.emit(WebSocketStatus::Opened);
        });

        let notify = notification.clone();
        websocket.add_event_listener(move |_: SocketCloseEvent| {
            notify.emit(WebSocketStatus::Closed);
        });

        let notify = notification.clone();
        websocket.add_event_listener(move |_: SocketErrorEvent| {
            notify.emit(WebSocketStatus::Error);
        });

        websocket.add_event_listener(move |event: SocketMessageEvent| {
            if let Some(data) = event.data().into_text() {
                callback.emit(data);
            }
        });

        Self { websocket, notification }
    }

    pub fn send(&mut self, json: &str) {
        if self.websocket.send_text(json).is_err() {
            self.notification.emit(WebSocketStatus::Error);
        }
    }
}

impl Task for WebSocketService {
    fn is_active(&self) -> bool {
        self.websocket.ready_state() == SocketReadyState::Open
    }

    fn cancel(&mut self) {
        self.websocket.close();
    }
}

impl Drop for WebSocketService {
    fn drop(&mut self) {
        if self.is_active() {
            self.cancel();
        }
    }
}
