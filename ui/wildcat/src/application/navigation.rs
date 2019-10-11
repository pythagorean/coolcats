use yew::prelude::*;

use wildcat_macros::ImplComponent;
use crate::application::{
    helpers::{link_to::*, application_helper::fa_icon},
    routes::RouterTarget,
};

#[derive(ImplComponent)]
pub struct Navigation;

impl Renderable<Navigation> for Navigation {
    fn view(&self) -> Html<Self> {
        html! {
            // navigation.items do |n|
                // n.item :web, safe_join([fa_icon('chevron-left fw'), t('settings.back')]), root_url
            <ul>
                <li id = "web">
                    {link_to(RouterTarget::SiteRoot, html! {<>
                        {fa_icon("chevron-left fw")}
                        {"Back to Mastodon"}
                    </>})}
                </li>
            </ul>
        }
    }
}
