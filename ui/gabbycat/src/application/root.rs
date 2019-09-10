use std::str::FromStr;
use strum_macros::{EnumString, IntoStaticStr};
use yew::prelude::*;

use super::{interfaces::home::Home, router};

pub struct Root;

#[derive(EnumString, IntoStaticStr)]
enum Route {
    #[strum(serialize = "/")]
    Site,
    #[strum(serialize = "/home")]
    Home,
}

impl Renderable<Root> for Root {
    fn view(&self) -> Html<Self> {
        let (route, _) = router::get();
        match Route::from_str(&route) {
            Ok(Route::Site) => {
                router::set(Route::Home.into(), "");
                self.view()
            }
            Ok(Route::Home) => html! {
                <Home />
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
