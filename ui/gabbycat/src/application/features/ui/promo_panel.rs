use yew::prelude::*;

use crate::application::facilities::{formatted_message::FormattedMessage, icon::Icon};

pub struct PromoPanel;

impl Renderable<PromoPanel> for PromoPanel {
    fn view(&self) -> Html<Self> {
        html! {
            <div class = "wtf-panel promo-panel">
                <div class = "promo-panel__container">
                    <div class = "promo-panel-item">
                        <a class = "promo-panel-item__btn", href = "https://blog.gab.com">
                            <Icon id = "align-left", class = "promo-panel-item__icon", fixed_width = true />
                            <FormattedMessage id = "promo-gab_news", default_message = "Gab News" />
                        </a>
                    </div>

                    <div class = "promo-panel-item">
                        <a class = "promo-panel-item__btn", href = "https://blog.gab.com/support-gab">
                            <Icon id = "users", class = "promo-panel-item__icon", fixed_width = true />
                            <FormattedMessage id = "promo-partners", default_message = "Affiliate Partners" />
                        </a>
                    </div>

                    <div class = "promo-panel-item">
                        <a class = "promo-panel-item__btn", href = "https://apps.gab.com">
                            <Icon id="th" class = "promo-panel-item__icon", fixed_width = true />
                            <FormattedMessage id = "promo-gab_apps", default_message = "Gab Apps" />
                        </a>
                    </div>
                </div>
            </div>
        }
    }
}

pub enum Msg {}

impl Component for PromoPanel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}
