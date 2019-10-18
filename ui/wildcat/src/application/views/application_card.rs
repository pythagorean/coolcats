use yew::prelude::*;

use wildcat_macros::ImplComponent;

#[derive(ImplComponent)]
pub struct ApplicationCard;

impl Renderable<ApplicationCard> for ApplicationCard {
    fn view(&self) -> Html<Self> {
        html! {
            // <% account_url = local_assigns[:admin] ? admin_account_path(account.id) : ActivityPub::TagManager.instance.url_for(account) %>
            <div class = "card h-card">
                //<%= link_to account_url, target: '_blank', rel: 'noopener' do %>
                    <div class = "card__img">
                        //<%= image_tag account.header.url, alt: '' %>
                    </div>
                    <div class = "card__bar">
                        <div class = "avatar">
                            //<%= image_tag account.avatar.url, alt: '', width: 48, height: 48, class: 'u-photo' %>
                        </div>
                        <div class = "display-name">
                            <span id = "default_account_display_name" style = "display: none">
                                //<%= account.username %>
                            </span>
                            <bdi>
                                <strong class = "emojify p-name">
                                    //<%= display_name(account, custom_emojify: true) %>
                                </strong>
                            </bdi>
                            <span>
                                //<%= acct(account) %>
                                //<%= fa_icon('lock') if account.locked? %>
                            </span>
                        </div>
                    </div>
                //<% end %>
            </div>
        }
    }
}
