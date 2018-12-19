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
    show: Option<ModelType>,
    partner: Option<Scope<Model>>,
}

pub enum Msg {
    SetModel(ModelType, Scope<Model>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            show: None,
            partner: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetModel(show, partner) => {
                self.show = Some(show);
                self.partner = Some(partner);
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        match self.show {
            Some(ModelType::Holoclient) => html! {
                <Holoclient: show=true,/>
            },

            Some(ModelType::App) => html! {
                <App: show=true,/>
            },

            None => html! { <div /> }
        }
    }
}
