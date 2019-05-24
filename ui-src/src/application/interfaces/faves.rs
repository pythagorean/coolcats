use yew::prelude::*;

use crate::application::{
    Action,
    context::{ self, ContextAgent },
    state::State,
};

interface_getstates!("favourites");
interface_component!(Faves, params, (u32, String), (0, String::new()));

// This will be mapped to Faves.local:
pub struct Local;

impl Local {
    fn new() -> Self {
        Self
    }
}

pub enum LocalMsg {
    NewStates,
    AddFavourite,
    RemoveFavourite,
}

impl Faves {
    fn local_update(&mut self, local_msg: LocalMsg) -> ShouldRender {
        match local_msg {
            LocalMsg::NewStates => {
                return true;
            }

            LocalMsg::AddFavourite => {
                let (_, address) = &self.params;
                let address = address.clone();
                self.update(Action::AddFavourite(address).into());
            }

            LocalMsg::RemoveFavourite => {
                let (_, address) = &self.params;
                let address = address.clone();
                self.update(Action::RemoveFavourite(address).into());
            }
        }
        false
    }
}

impl Renderable<Faves> for Faves {
    fn view(&self) -> Html<Self> {
        if self.getstate.is_empty() {
            return html! { <></> };
        };
        let favourites = self.getstate.strings("favourites");
        let (_, address) = &self.params;
        html! {<>
            <div>
                {if favourites.contains(address) {
                    html! {<>
                        <button
                            onclick=|_| LocalMsg::RemoveFavourite.into(),
                            class="glyphicon glyphicon-heart",
                            style="color: red;",
                        >
                    </>}
                } else {
                    html! {<>
                        <button
                            onclick=|_| LocalMsg::AddFavourite.into(),
                            class="glyphicon glyphicon-heart-empty",
                            style="color: red;",
                        >
                    </>}
                }}
            </div>
        </>}
    }
}
