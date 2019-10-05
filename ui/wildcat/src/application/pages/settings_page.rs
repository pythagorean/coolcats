use titlecase::titlecase;
use yew::prelude::*;

use wildcat_macros::ImplComponent;

#[derive(ImplComponent)]
pub struct SettingsPage;

struct SimpleForm {
    form_for: String,
}

impl SimpleForm {
    fn input(&self, name: &str, maxlength: u16) -> Html<SettingsPage> {
        let (name, render) = if name.contains(':') {
            let v: Vec<&str> = name.splitn(2, ':').collect();
            (v[0], v[1])
        } else {
            (name, name)
        };
        let form_for = self.form_for.clone() + "_" + name;
        let input_class = if maxlength < 500 {
            "string optional"
        } else {
            "text optional"
        };

        html! {
            <div class = format!("input with_label {} {}", input_class, form_for)>
                <div class = "label_input">
                    <label class = input_class, for = form_for>
                        {titlecase(render).replace("_", " ")}
                    </label>
                    <div class = "label_input__wrapper">
                        {if maxlength < 500 { html! {
                            <input
                                name = format!("{}[{}]", self.form_for, name),
                                id = form_for,
                                maxlength = maxlength,
                                class = input_class,
                                size = maxlength,
                                type = "text"
                            />
                        }} else { html! {
                            <textarea
                                name = format!("{}[{}]", self.form_for, name),
                                id = form_for,
                                maxlength = maxlength,
                                class = input_class,
                                type = "text"
                            />
                        }}}
                    </div>
                </div>
            </div>
        }
    }
}

impl Renderable<SettingsPage> for SettingsPage {
    fn view(&self) -> Html<Self> {
        let f = SimpleForm {
            form_for: "account".into(),
        };
        html! { <div> //From mastodon app/views/settings/profiles/show.html.haml
            /*
            //<% content_for :page_title do %>
                //<%= t('settings.edit_profile') %>
            //<% end %>*/

            //<%= simple_form_for @account, url: settings_profile_path, html: { method: :put } do |f| %>

            //<form>
                //<%= render 'shared/error_messages', object: @account %>
                <div class = "fields-row">
                    <div class = "fields-row__column fields-group fields-row__column-6">
                        //<%= f.input :display_name, wrapper: :with_label, input_html: { maxlength: 30 }, hint: false %>
                        {f.input("display_name", 30)}
                        //<%= f.input :note, wrapper: :with_label, input_html: { maxlength: 500 }, hint: false %>
                        {f.input("note:bio", 500)}
                    </div>
                </div>

                <div class = "fields-row">
                    <div class = "fields-row__column fields-row__column-6">
                        //<%= render 'application/card', account: @account %>
                    </div>
                    <div class = "fields-row__column fields-group fields-row__column-6">
                        //<%= f.input :header, wrapper: :with_label, input_html: { accept: AccountHeader::IMAGE_MIME_TYPES.join(',') }, hint: t('simple_form.hints.defaults.header', dimensions: '1500x500', size: number_to_human_size(AccountHeader::LIMIT)) %>
                        {f.input("header", 0)}
                        //<%= f.input :avatar, wrapper: :with_label, input_html: { accept: AccountAvatar::IMAGE_MIME_TYPES.join(',') }, hint: t('simple_form.hints.defaults.avatar', dimensions: '400x400', size: number_to_human_size(AccountAvatar::LIMIT)) %>
                        {f.input("avatar", 0)}
                    </div>
                </div>
                <hr class = "spacer"/>

                <div class = "fields-group">
                    //<%= f.input :locked, as: :boolean, wrapper: :with_label, hint: t('simple_form.hints.defaults.locked') %>
                    {f.input("locked", 0)}
                </div>
                <div class = "fields-group">
                    //<%= f.input :bot, as: :boolean, wrapper: :with_label, hint: t('simple_form.hints.defaults.bot') %>
                    {f.input("bot", 0)}
                </div>
                //<% if Setting.profile_directory %>
                    <div class = "fields-group">
                        //<%= f.input :discoverable, as: :boolean, wrapper: :with_label, hint: t('simple_form.hints.defaults.discoverable'), recommended: true %>
                        {f.input("discoverable", 0)}
                    </div>
                //<% end %>
                <hr class = "spacer"/>
                <div class = "fields-row">
                    <div class = "fields-row__column fields-group fields-row__column-6">
                        <div class = "input with_block_label">
                            <label>
                                //<%= t('simple_form.labels.defaults.fields') %>
                            </label>
                            <span class = "hint">
                                //<%= t('simple_form.hints.defaults.fields') %>
                            </span>
                            //<%= f.simple_fields_for :fields do |fields_f| %>
                                <div class = "row">
                                    //<%= fields_f.input :name, placeholder: t('simple_form.labels.account.fields.name'), input_html: { maxlength: 255 } %>
                                    //<%= fields_f.input :value, placeholder: t('simple_form.labels.account.fields.value'), input_html: { maxlength: 255 } %>
                                </div>
                            //<% end %>
                        </div>
                    </div>
                    <div class = "fields-row__column fields-group fields-row__column-6">
                        <h6>
                            //<%= t('verification.verification') %>
                        </h6>
                        <p class = "hint">
                            //<%= t('verification.explanation_html') %>
                        </p>
                        <div class = "input-copy">
                            <div class = "input-copy__wrapper">
                                <input type = "text", maxlength = 999, spellcheck = false, readonly = true/>
                                    //{type: :text, maxlength: '999', spellcheck: 'false', readonly: 'true', value: link_to('Mastodon', ActivityPub::TagManager.instance.url_for(@account), rel: 'me').to_str }
                            </div>
                            //<button type = "<%= :button %>">
                            <button>
                                //<%= t('generic.copy') %>
                            </button>
                        </div>
                    </div>
                </div>
                <div class = "actions">
                    //<%= f.button :button, t('generic.save_changes'), type: :submit %>
                    <button name = "button", type = "submit"/>
                </div>
            //</form>
            <hr/>
            <h6>
                //<%= t('auth.migrate_account') %>
            </h6>
            <p class = "muted-hint">
                //<%= t('auth.migrate_account_html', path: settings_migration_path) %>
            </p>
            <hr class = "spacer"/>
            <h6>
                //<%= t 'migrations.incoming_migrations' %>
            </h6>
            <p class = "muted-hint">
                //<%= t('migrations.incoming_migrations_html', path: settings_aliases_path) %>
            </p>
            //<% if open_deletion? %>
                <hr class = "spacer"/>
                <h6>
                    //<%= t('auth.delete_account') %>
                </h6>
                <p class = "muted-hint">
                    //<%= t('auth.delete_account_html', path: settings_delete_path) %>
                </p>
            //<% end %>
        </div> }
    }
}
