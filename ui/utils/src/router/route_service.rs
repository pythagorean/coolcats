use stdweb::web::window;

pub fn get() -> (String, String) {
    let location = window().location().unwrap();
    let pathname = location.pathname().unwrap();
    let hash = location.hash().unwrap();
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

pub fn set(route: &str, route_param: &str) {
    window().history().push_state((), "", Some(&format!("{}/{}", route, route_param)));
}
