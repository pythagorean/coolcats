use std::str::FromStr;
use strum_macros::{EnumString, IntoStaticStr};
use yew::prelude::*;

use super::{pages::home_page::HomePage, router};

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

pub enum Msg {}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}
