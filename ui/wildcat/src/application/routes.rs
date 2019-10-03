use std::str::FromStr;
use strum_macros::{EnumString, IntoStaticStr};
use yew::{prelude::*, agent::Dispatched};

use coolcats_ui_shared::{
    holoclient::{ToHoloclient, ToApplication},
    router::{Router, Route, Request as RouterRequest},
};
use super::pages::{home_page::HomePage, settings_page::SettingsPage};

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub params: Params,
    #[props(required)]
    pub callback: Callback<ToHoloclient>,
}

pub struct Routes {
    props: Props,
    router: Box<dyn Bridge<Router<()>>>,
    child: RouterTarget,
}

impl Renderable<Routes> for Routes {
    fn view(&self) -> Html<Self> {
        self.child.view()
    }
}

impl Renderable<Routes> for RouterTarget {
    fn view(&self) -> Html<Routes> {
        match self {
            RouterTarget::SiteRoot => {
                let route: Route<()> = Route::set(RouterTarget::HomePage.into(), "");
                Router::dispatcher().send(RouterRequest::ChangeRoute(route));
                html! {}
            }
            RouterTarget::HomePage => html! {
                <HomePage />
            },
            RouterTarget::SettingsPage => html! {
                <SettingsPage />
            },
            RouterTarget::Error => html! {
                <h1>{"404"}</h1>
            },
            RouterTarget::Unset => html! {},
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Params(pub ToApplication);

#[derive(EnumString, IntoStaticStr)]
pub enum RouterTarget {
    #[strum(serialize = "/")]
    SiteRoot,
    #[strum(serialize = "/home")]
    HomePage,
    #[strum(serialize = "/settings")]
    SettingsPage,
    #[strum(disabled = "true")]
    Error,
    #[strum(disabled = "true")]
    Unset,
}

pub enum Msg {
    Route(Route<()>),
}

impl Component for Routes {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let router = Router::bridge(link.send_back(Msg::Route));
        let child = RouterTarget::Unset;
        Self {
            props,
            router,
            child,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.router.send(RouterRequest::GetCurrentRoute);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Route(route) => {
                self.child = RouterTarget::from_str(&route.route).unwrap_or(RouterTarget::Error);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}
