#![recursion_limit="128"]
#[macro_use]
extern crate yew;
use yew::prelude::*;

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

pub struct Model;

pub enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <Holoclient: show=true,/>
            //<App: show=true,/>
        }
    }
}
