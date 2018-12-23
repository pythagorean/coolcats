#![recursion_limit="128"]
#[macro_use]
extern crate yew;
use yew::{
    prelude::*,
    html::Scope,
};

#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate serde_derive;

mod components;
mod settings;
mod holoclient;
mod app;

use crate::{
    holoclient::Holoclient,
    app::App,
};

pub enum ModelType {
    Holoclient,
    App
}

pub struct Model {
    model_type: Option<ModelType>,
    partner: Option<Scope<Model>>,
    params: String,
}

#[derive(Debug)]
pub enum ToHoloclient {
    Msg(String),
}

pub enum Msg {
    SetModel(ModelType, Scope<Model>),
    FromHoloclient(String),
    FromApp(ToHoloclient),
    ToHoloclient(ToHoloclient),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            model_type: None,
            partner: None,
            params: "".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetModel(model_type, partner) => {
                self.model_type = Some(model_type);
                self.partner = Some(partner);
            },
            Msg::FromHoloclient(text) => {
                js! { alert(@{
                    format!{"Holoclient: {}", text}
                })};
            },
            Msg::FromApp(msg) => {
                if let Some(ModelType::App) = self.model_type {
                    self.partner.as_mut().unwrap().send_message(Msg::ToHoloclient(msg));
                    return false;
                } else {
                    panic!("FromApp not sent from App");
                }
            },
            Msg::ToHoloclient(msg_from_app) => {
                if let Some(ModelType::Holoclient) = self.model_type {
                    match msg_from_app {
                        ToHoloclient::Msg(msg) => self.params = msg,
                    }
                } else {
                    panic!("ToHoloclient not sent to Holoclient");
                }
            },
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        match self.model_type {
            Some(ModelType::Holoclient) => html! {
                <Holoclient:
                    params = self.params.clone(),
                    root_callback = |data| Msg::FromHoloclient(data),
                />
            },

            Some(ModelType::App) => html! {
                <App: to_model=|data| Msg::FromApp(data),/>
            },

            None => html! { <div /> }
        }
    }
}
