use yew::prelude::*;

use wildcat_macros::PropsComponent;
use crate::application::helpers::image_tag::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub account: String,
}

#[derive(PropsComponent)]
pub struct ApplicationCard {
    props: Props,
}

impl Renderable<ApplicationCard> for ApplicationCard {
    fn view(&self) -> Html<Self> {
        let account = &self.props.account;
        let account_url = format!("/@{}", account);

        html! {
            <div class = "card h-card">
                //<%= link_to account_url, target: '_blank', rel: 'noopener' do %>
                <a href = account_url, target = "_blank", rel = "noopener">
                    <div class = "card__img">
                        //<%= image_tag account.header.url, alt: '' %>
                        {image_tag("/headers/original/missing.png", "")}
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
                </a>
            </div>
        }
    }
}
