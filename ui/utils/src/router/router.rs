use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use stdweb::{JsSerialize, Value, unstable::TryFrom};
use yew::worker::*;

use super::{route::Route, route_service::RouteService};

pub struct Router<T>
where
    for<'de> T:
        'static + JsSerialize + Clone + TryFrom<Value> + Default + Serialize + Deserialize<'de>,
{
    link: AgentLink<Router<T>>,
    route_service: RouteService<T>,
    subscribers: HashSet<HandlerId>,
}

pub enum Msg<T>
where
    T: 'static,
{
    BrowserNavigationRouteChanged(T),
}

#[derive(Serialize, Deserialize)]
pub enum Request<T> {
    ChangeRoute(Route<T>),
    ChangeRouteNoBroadcast(Route<T>),
    GetCurrentRoute,
}

impl<T> Transferable for Request<T> where for<'de> T: Serialize + Deserialize<'de> {}

impl<T> Agent for Router<T>
where
    for<'de> T:
        'static + JsSerialize + Clone + TryFrom<Value> + Default + Serialize + Deserialize<'de>,
{
    type Reach = Context;
    type Message = Msg<T>;
    type Input = Request<T>;
    type Output = Route<T>;

    fn create(link: AgentLink<Self>) -> Self {
        let callback = link.send_back(Msg::BrowserNavigationRouteChanged);
        let mut route_service = RouteService::new();
        route_service.register_callback(callback);

        Self {
            link,
            route_service,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::BrowserNavigationRouteChanged(state) => {
                let mut route = Route::current_route(&self.route_service);
                route.state = state;
                for sub in self.subscribers.iter() {
                    self.link.response(*sub, route.clone());
                }
            }
        }
    }

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::ChangeRoute(route) => {
                let route_string: String = route.to_route_string();
                self.route_service.set_route(&route_string, route.state);
                let route = Route::current_route(&self.route_service);
                for sub in self.subscribers.iter() {
                    self.link.response(*sub, route.clone());
                }
            }
            Request::ChangeRouteNoBroadcast(route) => {
                let route_string: String = route.to_route_string();
                self.route_service.set_route(&route_string, route.state);
            }
            Request::GetCurrentRoute => {
                let route = Route::current_route(&self.route_service);
                self.link.response(who, route.clone());
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.link.response(id, Route::current_route(&self.route_service));
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
