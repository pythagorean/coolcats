#![recursion_limit = "128"]
#[macro_use]
extern crate yew;
use yew::{
    prelude::*,
    html::Scope,
};

#[macro_use]
extern crate stdweb;

extern crate json;

extern crate strum;
#[macro_use]
extern crate strum_macros;

mod holoclient;
mod app;
mod utils;

use self::{
    holoclient::{Holoclient, ToHoloclient},
    app::{App, ToApp},
};

pub enum ModelType {
    Holoclient,
    App,
}

pub struct Model {
    model_type: Option<ModelType>,
    partner: Option<Scope<Model>>,
    holoclient_params: holoclient::Params,
    app_params: app::Params,
}

pub enum Msg {
    SetModel(ModelType, Scope<Model>),
    FromApp(ToHoloclient),
    ToHoloclient(ToHoloclient),
    FromHoloclient(ToApp),
    ToApp(ToApp),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            model_type: None,
            partner: None,
            holoclient_params: holoclient::Params::new(),
            app_params: app::Params(ToApp::None),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetModel(model_type, partner) => {
                if self.model_type.is_none() {
                    self.model_type = Some(model_type);
                    self.partner = Some(partner);
                } else {
                    panic! { "Msg::SetModel received within already defined model" };
                }
            }

            Msg::FromApp(msg) => {
                if let Some(ModelType::App) = self.model_type {
                    self.app_params = app::Params(ToApp::None);
                    self.partner.as_mut().unwrap().send_message(Msg::ToHoloclient(msg));
                } else {
                    panic! { "Msg::FromApp not received in App" };
                }
            }

            Msg::ToHoloclient(params_from_app) => {
                if let Some(ModelType::Holoclient) = self.model_type {
                    let ToHoloclient::Call(params) = params_from_app;
                    self.holoclient_params = params;
                } else {
                    panic! { "Msg::ToHoloclient not received in Holoclient" };
                }
            }

            Msg::FromHoloclient(msg) => {
                if let Some(ModelType::Holoclient) = self.model_type {
                    self.holoclient_params.clear();
                    self.partner.as_mut().unwrap().send_message(Msg::ToApp(msg));
                } else {
                    panic! { "Msg::FromHoloclient not received in Holoclient" };
                }
            }

            Msg::ToApp(params_from_holoclient) => {
                if let Some(ModelType::App) = self.model_type {
                    self.app_params = app::Params(params_from_holoclient);
                } else {
                    panic! { "Msg::ToApp not received in App" };
                }
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        match self.model_type {
            Some(ModelType::Holoclient) => html! {
                <Holoclient:
                    params = self.holoclient_params.clone(),
                    callback = Msg::FromHoloclient,
                />
            },

            Some(ModelType::App) => html! {
                <App:
                    params = self.app_params.clone(),
                    callback = Msg::FromApp,
                />
            },

            None => html! { <></> },
        }
    }
}
