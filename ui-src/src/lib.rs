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

pub enum ToHoloclient {
    Msg(String),
}

#[derive(Debug)]
pub enum ToApp {
    Msg(String),
}

pub enum Msg {
    SetModel(ModelType, Scope<Model>),
    FromApp(ToHoloclient),
    ToHoloclient(ToHoloclient),
    FromHoloclient(String),
    ToApp(String),
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
            Msg::FromApp(msg) => {
                if let Some(ModelType::App) = self.model_type {
                    self.partner.as_mut().unwrap().send_message(Msg::ToHoloclient(msg));
                    return false;
                } else {
                    panic!("Msg::FromApp not received in App");
                }
            },
            Msg::ToHoloclient(msg_from_app) => {
                if let Some(ModelType::Holoclient) = self.model_type {
                    let ToHoloclient::Msg(msg) = msg_from_app;
                    self.params = msg;
                } else {
                    panic!("Msg::ToHoloclient not received in Holoclient");
                }
            },
            Msg::FromHoloclient(text) => {
                if let Some(ModelType::Holoclient) = self.model_type {
                    self.partner.as_mut().unwrap().send_message(Msg::ToApp(text));
                    self.params = "".into();
                } else {
                    panic!("Msg::FromHoloclient not received in Holoclient");
                }
            },
            Msg::ToApp(msg_from_holoclient) => {
                if let Some(ModelType::App) = self.model_type {
                    let text = msg_from_holoclient;
                    js! { alert(@{
                        format!{"Msg::ToApp received {:?}", text}
                    })};
                } else {
                    panic!("Msg::ToApp not received in App");
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
                    params = self.params.clone(),
                    root_callback = |data| Msg::FromHoloclient(data),
                />
            },

            Some(ModelType::App) => html! {
                <App:
                    params = self.params.clone(),
                    root_callback = |data| Msg::FromApp(data),
                />
            },

            None => html! { <div /> }
        }
    }
}
