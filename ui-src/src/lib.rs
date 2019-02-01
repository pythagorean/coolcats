#![recursion_limit = "128"]
#[macro_use]
extern crate yew;
use yew::{
    prelude::*,
    html::Scope,
};

#[macro_use]
extern crate yew_router;

#[macro_use]
extern crate stdweb;

extern crate json;

extern crate strum;
#[macro_use]
extern crate strum_macros;

mod holoclient;
mod application;
mod utils;

use self::{
    holoclient::{Holoclient, ToHoloclient},
    application::{Application, ToApplication},
};

pub enum ModelType {
    Holoclient,
    Application,
}

pub struct Model {
    model_type: Option<ModelType>,
    partner: Option<Scope<Model>>,
    holoclient_params: holoclient::Params,
    application_params: application::Params,
}

pub enum Msg {
    SetModel(ModelType, Scope<Model>),
    FromApplication(ToHoloclient),
    ToHoloclient(ToHoloclient),
    FromHoloclient(ToApplication),
    ToApplication(ToApplication),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            model_type: None,
            partner: None,
            holoclient_params: holoclient::Params::new(),
            application_params: application::Params(ToApplication::None),
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

            Msg::FromApplication(msg) => {
                if let Some(ModelType::Application) = self.model_type {
                    self.application_params = application::Params(ToApplication::None);
                    self.partner.as_mut().unwrap().send_message(Msg::ToHoloclient(msg));
                } else {
                    panic! { "Msg::FromApplication not received in Application" };
                }
            }

            Msg::ToHoloclient(params_from_application) => {
                if let Some(ModelType::Holoclient) = self.model_type {
                    let ToHoloclient::Call(params) = params_from_application;
                    self.holoclient_params = params;
                } else {
                    panic! { "Msg::ToHoloclient not received in Holoclient" };
                }
            }

            Msg::FromHoloclient(msg) => {
                if let Some(ModelType::Holoclient) = self.model_type {
                    self.holoclient_params.clear();
                    self.partner.as_mut().unwrap().send_message(Msg::ToApplication(msg));
                } else {
                    panic! { "Msg::FromHoloclient not received in Holoclient" };
                }
            }

            Msg::ToApplication(params_from_holoclient) => {
                if let Some(ModelType::Application) = self.model_type {
                    self.application_params = application::Params(params_from_holoclient);
                } else {
                    panic! { "Msg::ToApplication not received in Application" };
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

            Some(ModelType::Application) => html! {
                <Application:
                    params = self.application_params.clone(),
                    callback = Msg::FromApplication,
                />
            },

            None => html! { <></> },
        }
    }
}
