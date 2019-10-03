use std::marker::PhantomData;
use stdweb::{
    JsSerialize, Value,
    web::{window, Location, History, EventListenerHandle, IEventTarget, event::PopStateEvent},
    unstable::TryFrom,
};
use yew::callback::Callback;

pub struct RouteService<T> {
    location: Location,
    history: History,
    event_listener: Option<EventListenerHandle>,
    phantom_data: PhantomData<T>,
}

impl<T> RouteService<T>
where
    T: 'static + JsSerialize + TryFrom<Value>,
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            location: window().location().unwrap(),
            history: window().history(),
            event_listener: None,
            phantom_data: PhantomData,
        }
    }

    pub fn register_callback(&mut self, callback: Callback<T>) {
        self.event_listener = Some(window().add_event_listener(move |event: PopStateEvent| {
            let state_value: Value = event.state();
            if let Ok(state) = T::try_from(state_value) {
                callback.emit(state);
            }
        }));
    }

    pub fn get_route_and_param(&self) -> (String, String) {
        let pathname = self.location.pathname().unwrap();
        let hash = self.location.hash().unwrap();
        let mut route = if hash.is_empty() {
            pathname.as_str()
        } else {
            &hash[1..]
        };
        let mut route_param = "";
        if route.len() > 1 {
            if let Some(n) = &route[1..].find('/') {
                let (r, p) = route.split_at(n + 1);
                route = r;
                route_param = &p[1..];
            }
        }
        (route.to_string(), route_param.to_string())
    }

    pub fn set_route_and_param(&self, route: &str, route_param: &str, state: T) {
        if route_param.is_empty() {
            self.set_route(route, state);
        } else {
            self.history.push_state(state, "", Some(&[route, route_param].join("/")));
        }
    }

    pub fn set_route(&self, route: &str, state: T) {
        self.history.push_state(state, "", Some(route));
    }
}
