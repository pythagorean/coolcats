use std::collections::HashMap;
use yew::prelude::*;

use wildcat_macros::{LocaleComponent, UsesLocaleValues, use_locale_values};
use crate::application::{
    context,
    helpers::{link_to::*, application_helper::fa_icon},
    routes::RouterTarget,
};

use_locale_values!["settings-back"];

#[derive(LocaleComponent, UsesLocaleValues)]
pub struct Navigation {
    context: Box<dyn Bridge<context::Worker>>,
    locale_values: HashMap<String, String>,
}

impl Renderable<Navigation> for Navigation {
    fn view(&self) -> Html<Self> {
        let t = |message_id| self.get_locale_value(message_id);
        html! {
            // navigation.items do |n|
                // n.item :web, safe_join([fa_icon('chevron-left fw'), t('settings.back')]), root_url
            <ul>
                <li id = "web">
                    {link_to_target(RouterTarget::SiteRoot, html! {<>
                        {fa_icon("chevron-left fw")}
                        {t("settings-back")}
                    </>})}
                </li>
            </ul>
        }
    }
}
