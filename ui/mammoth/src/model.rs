use yew::prelude::*;

use mammoth_macros::ImplComponent;
use crate::application::Application;

#[derive(ImplComponent)]
pub struct Model;

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <Application />
        }
    }
}
