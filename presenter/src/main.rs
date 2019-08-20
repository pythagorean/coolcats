use gotham::{
    handler::assets::FileOptions,
    router::builder::{build_simple_router, DrawRoutes, DefineSingleRoute},
};

pub fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Need to pass an arg which is the path to serve");
    let index = path.clone() + "/index.html";
    let addr = "127.0.0.1:8000";
    println!(
        "Listening for requests at http://{} from path {:?}",
        addr, path
    );
    let router = build_simple_router(|route| {
        route.get("/").to_file(&index);
        route.get("/*").to_dir(
            FileOptions::new(&path)
                .with_gzip(true)
                .build(),
        );
    });
    gotham::start(addr, router)
}
