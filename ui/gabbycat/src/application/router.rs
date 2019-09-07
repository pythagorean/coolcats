use stdweb::web::window;

pub fn get() -> (String, String) {
    let pathname = window().location().unwrap().pathname().unwrap();
    let mut route = pathname.as_str();
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

pub fn set(route: &str, route_param: &str) {
    window()
        .history()
        .push_state((), "", Some(&format!("{}/{}", route, route_param)));
}
