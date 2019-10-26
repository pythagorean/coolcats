use std::collections::HashMap;
use yew::prelude::*;

use wildcat_macros::{LocaleComponent, UsesLocaleValues, use_locale_values};
use crate::application::{
    context,
    helpers::{link_to::*, application_helper::fa_icon},
    routes::RouterTarget,
};

use_locale_values![
    "settings-back",
    "settings-profile",
    "settings-appearance",
    "settings-featured_tags",
    "settings-preferences",
    "settings-relationships",
    "filters-index-title",
    "settings-account",
    "settings-import_and_export",
    "settings-development",
    "moderation-title",
    "admin-title",
    "auth-logout"
];

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
                // vvv These are incomplete static elements vvv
                <li id = "profile", class = "selected">
                    <a class = "selected", href = "/settings/profile">
                        {fa_icon("user fw")}
                        {t("settings-profile")}
                    </a>
                    <ul>
                        <li id = "profile", class = "selected simple-navigation-active-leaf">
                            <a class = "selected", href = "/settings/profile">
                                {fa_icon("pencil fw")}
                                {t("settings-appearance")}
                            </a>
                        </li>
                        <li id = "featured_tags">
                            <a href = "/settings/featured_tags">
                                {fa_icon("hashtag fw")}
                                {t("settings-featured_tags")}
                            </a>
                        </li>
                    </ul>
                </li>
                <li id = "preferences">
                    <a href = "/settings/preferences">
                        {fa_icon("cog fw")}
                        {t("settings-preferences")}
                    </a>
                </li>
                <li id = "relationships">
                    <a href = "/relationships">
                        {fa_icon("users fw")}
                        {t("settings-relationships")}
                    </a>
                </li>
                <li id = "filters">
                    <a href = "/filters">
                        {fa_icon("filter fw")}
                        {t("filters-index-title")}
                    </a>
                </li>
                <li id = "security">
                    <a href = "/auth/edit">
                        {fa_icon("lock fw")}
                        {t("settings-account")}
                    </a>
                </li>
                <li id = "data">
                    <a href = "/settings/export">
                        {fa_icon("cloud-download fw")}
                        {t("settings-import_and_export")}
                    </a>
                </li>
                <li id = "development">
                    <a href = "/settings/applications">
                        {fa_icon("code fw")}
                        {t("settings-development")}
                    </a>
                </li>
                <li id = "moderation">
                    <a href = "/admin/reports">
                        {fa_icon("gavel fw")}
                        {t("moderation-title")}
                    </a>
                </li>
                <li id = "admin">
                    <a href = "/admin/dashboard">
                        {fa_icon("cogs fw")}
                        {t("admin-title")}
                    </a>
                </li>
                <li id = "logout">
                    <a data-method = "delete", href = "/auth/signout">
                        {fa_icon("sign-out fw")}
                        {t("auth-logout")}
                    </a>
                </li>
            </ul>
        }
    }
}
