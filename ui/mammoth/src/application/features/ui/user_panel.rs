use yew::prelude::*;

use mammoth_macros::ImplComponent;
use crate::application::facilities::{avatar::Avatar, formatted_message::FormattedMessage};

#[derive(ImplComponent)]
pub struct UserPanel;

impl Renderable<UserPanel> for UserPanel {
    fn view(&self) -> Html<Self> {
        let account = "";

        html! {
            <div class = "user_panel">
                <div class = "user-panel__container">
                    <div class = "user-panel__header">
                        //<img src={autoPlayGif ? account.get('header') : account.get('header_static')} alt='' />
                    </div>

                    <div class = "user-panel__profile">
                        //<Link to={`/${account.get('acct')}`} title={acct}>
                            <Avatar account = account />
                        //</Link>
                    </div>

                    <div class = "user-panel__meta">

                        <div class = "user-panel__account">
                            <h1>
                                //<Link to={`/${account.get('acct')}`}>
                                    <span class = "user-panel__account__name" />//, dangerouslySetInnerHTML={displayNameHtml} />
                                    <small class = "user-panel__account__username">/*@{acct}*/</small>
                                //</Link>
                            </h1>
                        </div>

                        <div class = "user-panel__stats-block">

                            <div class = "user-panel-stats-item">
                                //<Link to={`/${account.get('acct')}`} title={intl.formatNumber(account.get('statuses_count'))}>
                                    <strong class = "user-panel-stats-item__value">/*{shortNumberFormat(account.get('statuses_count'))}*/</strong>
                                    <span class = "user-panel-stats-item__label">
                                        <FormattedMessage /*class = "user-panel-stats-item__label",*/ id = "account-posts", default_message = "Toots" />
                                    </span>
                                //</Link>
                            </div>

                            <div className = "user-panel-stats-item">
                                //<Link to={`/${account.get('acct')}/followers`} title={intl.formatNumber(account.get('followers_count'))}>
                                    <strong class = "user-panel-stats-item__value">/*{shortNumberFormat(account.get('followers_count'))}*/</strong>
                                    <span class = "user-panel-stats-item__label">
                                        <FormattedMessage id = "account-followers", default_message="Followers" />
                                    </span>
                                //</Link>
                            </div>

                            <div class = "user-panel-stats-item">
                                //<Link to={`/${account.get('acct')}/following`} title={intl.formatNumber(account.get('following_count'))}>
                                    <strong class = "user-panel-stats-item__value">/*{shortNumberFormat(account.get('following_count'))}*/</strong>
                                    <span class = "user-panel-stats-item__label">
                                        <FormattedMessage /*class = "user-panel-stats-item__label",*/ id = "account-follows", default_message = "Follows" />
                                    </span>
                                //</Link>
                            </div>

                        </div>

                    </div>

                </div>
            </div>
        }
    }
}
