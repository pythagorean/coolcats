use std::str::FromStr;
use strum_macros::{EnumString, IntoStaticStr};
use yew::prelude::*;

use mammoth_macros::ImplComponent;
use super::{pages::home_page::HomePage, router};

#[derive(ImplComponent)]
pub struct Root;

#[derive(EnumString, IntoStaticStr)]
enum Route {
    #[strum(serialize = "/")]
    SiteRoot,
    #[strum(serialize = "/home")]
    HomePage,
}

impl Renderable<Root> for Root {
    fn view(&self) -> Html<Self> {
        let (route, _) = router::get();
        match Route::from_str(&route) {
            Ok(Route::SiteRoot) => {
                router::set(Route::HomePage.into(), "");
                self.view()
            }
            Ok(Route::HomePage) => html! {
                <HomePage />
            },
            Err(_) => html! {
                <h1>{"404"}</h1>
            },
        }
    }
}
