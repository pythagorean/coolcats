use titlecase::titlecase;
use stdweb::{js, web::document};
use yew::prelude::*;

use crate::application::{
    helpers::{link_to::*, application_helper::fa_icon, statuses_helper::svg_logo_full},
    navigation::Navigation,
    routes::RouterTarget,
};

pub fn admin_wrap<T: Component>(page_title: &str, html: Html<T>) -> Html<T> {
    //From mastodon app/views/layouts/admin.html.haml
    let page_title = titlecase(page_title).replace("_", " ");
    document().set_title(&format!("{} - Coolcats", page_title));
    js! {  @(no_return) document.body.className = "admin theme-default no-reduce-motion" };
    html! {
        //<% content_for :header_tags do %>
            //<%= javascript_pack_tag 'public', integrity: true, crossorigin: 'anonymous' %>
        //<% end %>
        //<% content_for :content do %>
        <div class = "admin-wrapper">
            <div class = "sidebar-wrapper">
                <div class = "sidebar-wrapper__inner">
                    <div class = "sidebar">
                        {link_to_target(RouterTarget::SiteRoot, html! {
                            <img class = "logo", alt = "Coolcats", src = "/images/logo.svg"/>
                        })}
                        <div class = "sidebar__toggle">
                            <div class = "sidebar__toggle__logo">
                                {link_to_target(RouterTarget::SiteRoot, svg_logo_full())}
                            </div>
                            {link_to_class("sidebar__toggle__icon", fa_icon("bars"))}
                        </div>
                        <Navigation />
                    </div>
                </div>
            </div>
            <div class = "content-wrapper">
                <div class = "content">
                    <h2>
                        {page_title}
                    </h2>
                    //<%= render 'application/flashes' %>
                    //<%= yield %>
                    {html}
                </div>
            </div>
            <div class = "sidebar-wrapper sidebar-wrapper--empty"></div>
        </div>
        //<%= render template: 'layouts/application' %>
    }
}
