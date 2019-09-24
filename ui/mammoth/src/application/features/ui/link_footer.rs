use yew::prelude::*;

use gabbycat_macros::ImplComponent;
use crate::application::facilities::formatted_message::FormattedMessage;

#[derive(ImplComponent)]
pub struct LinkFooter;

impl Renderable<LinkFooter> for LinkFooter {
    fn view(&self) -> Html<Self> {
        let invites_enabled = false;
        let account = false;

        html! {
            <div class = "getting-started__footer">
                <ul>
                    {if invites_enabled && account { html! {
                        <li><a href = "/invites"><FormattedMessage id = "getting_started-invite", default_message = "Invite people" /></a></li>
                    }} else { html! {} }}
                    //{account && <li><a href='#' onClick={onOpenHotkeys}><FormattedMessage id='navigation_bar.keyboard_shortcuts' defaultMessage='Hotkeys' /></a> · </li>}
                    //{account && <li><a href='/auth/edit'><FormattedMessage id='getting_started.security' defaultMessage='Security' /></a> · </li>}
                    <li><a href = "/about"><FormattedMessage id = "navigation_bar-info", default_message = "About" /></a>{" · "}</li>
                    <li><a href = "/settings/applications"><FormattedMessage id = "getting_started-developers", default_message = "Developers" /></a>{" · "}</li>
                    <li><a href = "/about/tos"><FormattedMessage id = "getting_started-terms", default_message = "Terms of Service" /></a>{" · "}</li>
                    <li><a href = "/about/dmca"><FormattedMessage id = "getting_started-dmca", default_message = "DMCA" /></a>{" · "}</li>
                    <li><a href = "/about/sales"><FormattedMessage id = "getting_started-terms_of_sale", default_message="Terms of Sale" /></a>{" · "}</li>
                    <li><a href = "/about/privacy"><FormattedMessage id = "getting_started-privacy", default_message = "Privacy Policy" /></a></li>
                    //{account && <li> · <a href='/auth/sign_out' data-method='delete'><FormattedMessage id='navigation_bar.logout' defaultMessage='Logout' /></a></li>}
                </ul>

                <p>
                    <FormattedMessage
                        id = "getting_started-open_source_notice",
                        default_message = "Gabbycat is open source software. You can contribute or report issues on our GitHub repository."
                    />
                </p>
                <p>{"© 2019 Michael Goldman"}</p>
            </div>
        }
    }
}
