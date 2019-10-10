use titlecase::titlecase;
use yew::prelude::*;

use crate::application::helpers::{link_to::link_to, statuses_helper::svg_logo_full};

pub fn admin_wrap<T: Component>(page_title: &str, html: Html<T>) -> Html<T> {
    //From mastodon app/views/layouts/admin.html.haml
    html! {
        //<% content_for :header_tags do %>
            //<%= javascript_pack_tag 'public', integrity: true, crossorigin: 'anonymous' %>
        //<% end %>
        //<% content_for :content do %>
        <div class = "admin-wrapper">
            <div class = "sidebar-wrapper">
                <div class = "sidebar-wrapper__inner">
                    <div class = "sidebar">
                        {link_to("/", html! {
                            //<%= image_pack_tag 'logo.svg', class: 'logo', alt: 'Mastodon' %>
                        })}
                        <div class = "sidebar__toggle">
                            <div class = "sidebar__toggle__logo">
                                {link_to("/", svg_logo_full())}
                            </div>
                            //<%= link_to '#', class: 'sidebar__toggle__icon' do %>
                            <a href = "#", class = "sidebar__toggle__icon">
                                //<%= fa_icon 'bars' %>
                                <i class = "fa fa-bars"></i>
                            </a>
                        </div>
                        //<%= render_navigation %>
                    </div>
                </div>
            </div>
            <div class = "content-wrapper">
                <div class = "content">
                    <h2>
                        //<%= yield :page_title %>
                        {titlecase(page_title).replace("_", " ")}
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
