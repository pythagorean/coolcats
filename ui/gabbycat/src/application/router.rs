use stdweb::web::window;

pub fn get() -> (String, String) {
    let fragment = window().location().unwrap().hash().unwrap();
    let mut route = if fragment.is_empty() {
        "/"
    } else {
        &fragment[1..]
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
    js! {
        window.location.hash = @{format!("{}/{}", route, route_param)}
    }
}
