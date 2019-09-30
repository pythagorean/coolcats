use serde::{Deserialize, Serialize};
use stdweb::{JsSerialize, Value, unstable::TryFrom};
use yew::worker::*;

use super::route_service::RouteService;

#[derive(Clone, Serialize, Deserialize)]
pub struct Route<T> {
    pub route: String,
    pub route_param: String,
    pub state: T,
}

impl<T> Transferable for Route<T> where for<'de> T: Serialize + Deserialize<'de> {}

impl<T> Route<T>
where
    T: 'static + JsSerialize + Clone + TryFrom<Value> + Default,
{
    pub fn current_route(route_service: &RouteService<T>) -> Self {
        let (route, route_param) = route_service.get_route_and_param();
        Self {
            route,
            route_param,
            state: T::default(),
        }
    }

    pub fn to_route_string(&self) -> String {
        [self.route.clone(), self.route_param.clone()].join("/")
    }

    pub fn set(route: &str, route_param: &str) -> Self {
        Self {
            route: route.into(),
            route_param: route_param.into(),
            state: T::default()
        }
    }
}
