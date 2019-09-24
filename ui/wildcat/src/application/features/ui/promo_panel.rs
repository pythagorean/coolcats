use yew::prelude::*;

use wildcat_macros::ImplComponent;
use crate::application::facilities::{formatted_message::FormattedMessage, icon::Icon};

#[derive(ImplComponent)]
pub struct PromoPanel;

impl Renderable<PromoPanel> for PromoPanel {
    fn view(&self) -> Html<Self> {
        html! {
            <div class = "wtf-panel promo-panel">
                <div class = "promo-panel__container">
                    <div class = "promo-panel-item">
                        <a class = "promo-panel-item__btn", href = "https://">
                            <Icon id = "align-left", class = "promo-panel-item__icon", fixed_width = true />
                            <FormattedMessage id = "promo-wildcat_news", default_message = "Coolcats News" />
                        </a>
                    </div>

                    <div class = "promo-panel-item">
                        <a class = "promo-panel-item__btn", href = "https://">
                            <Icon id = "users", class = "promo-panel-item__icon", fixed_width = true />
                            <FormattedMessage id = "promo-partners", default_message = "Affiliate Partners" />
                        </a>
                    </div>

                    <div class = "promo-panel-item">
                        <a class = "promo-panel-item__btn", href = "https://">
                            <Icon id="th" class = "promo-panel-item__icon", fixed_width = true />
                            <FormattedMessage id = "promo-gab_apps", default_message = "Coolcats Apps" />
                        </a>
                    </div>
                </div>
            </div>
        }
    }
}
