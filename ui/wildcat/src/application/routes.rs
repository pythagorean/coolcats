use std::str::FromStr;
use strum_macros::{EnumString, IntoStaticStr};
use yew::prelude::*;

use coolcats_ui_shared::{
    holoclient::{ToHoloclient, ToApplication},
    router::RouteService,
};
use wildcat_macros::PropsComponent;
use super::pages::home_page::HomePage;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub params: Params,
    #[props(required)]
    pub callback: Callback<ToHoloclient>,
}

#[derive(PropsComponent)]
pub struct Routes {
    props: Props,
}

impl Renderable<Routes> for Routes {
    fn view(&self) -> Html<Self> {
        let route_service: RouteService<()> = RouteService::new();
        let (route, _) = route_service.get_route_and_param();
        match Route::from_str(&route) {
            Ok(route) => match route {
                Route::SiteRoot => {
                    route_service.set_route(Route::HomePage.into(), ());
                    self.view()
                }
                Route::HomePage => html! {
                    <HomePage />
                }
            }
            Err(_) => html! {
                <h1>{"404"}</h1>
            },
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Params(pub ToApplication);

#[derive(EnumString, IntoStaticStr)]
enum Route {
    #[strum(serialize = "/")]
    SiteRoot,
    #[strum(serialize = "/home")]
    HomePage,
}
