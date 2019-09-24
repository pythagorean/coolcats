use yew::prelude::*;

use wildcat_macros::ImplComponent;
use crate::application::facilities::{formatted_message::FormattedMessage, icon::Icon};

#[derive(ImplComponent)]
pub struct WhoToFollowPanel;

impl Renderable<WhoToFollowPanel> for WhoToFollowPanel {
    fn view(&self) -> Html<Self> {
        html! {
            <div class = "wtf-panel">
                <div class = "wtf-panel-header">
                    <Icon id = "users", class = "wtf-panel-header__icon" />
                    <span class = "wtf-panel-header__label">
                        <FormattedMessage id = "who_to_follow-title", default_message = "Who To Follow" />
                    </span>
                </div>
                <div class = "wtf-panel__content">
                    <div class = "wtf-panel__list">
                        /*
                        {suggestions && suggestions.map(accountId => (
                          <AccountContainer
                            key={accountId}
                            id={accountId}
                            actionIcon='times'
                            actionTitle={intl.formatMessage(messages.dismissSuggestion)}
                            onActionClick={dismissSuggestion}
                          />
                        ))}
                        */
                    </div>
                </div>
            </div>
        }
    }
}
