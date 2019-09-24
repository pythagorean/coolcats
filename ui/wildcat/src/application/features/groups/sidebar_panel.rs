use yew::prelude::*;

use mammoth_macros::ImplComponent;
use crate::application::facilities::{formatted_message::FormattedMessage, icon::Icon};

#[derive(ImplComponent)]
pub struct GroupSidebarPanel;

impl Renderable<GroupSidebarPanel> for GroupSidebarPanel {
    fn view(&self) -> Html<Self> {
        html! {
            <div class = "wtf-panel group-sidebar-panel">
                <div class = "wtf-panel-header">
                    <Icon id = "users", class = "wtf-panel-header__icon" />
                    <span class = "wtf-panel-header__label">
                        <FormattedMessage id = "groups-sidebar-panel-title", default_message = "Groups You're In" />
                    </span>
                </div>

                <div class = "wtf-panel__content">
                    <div class = "group-sidebar-panel__items">
                        //{groupIds.slice(0, 10).map(groupId => <Item key={groupId} id={groupId} />)}
                        //{count > 10 && <Link className="group-sidebar-panel__items__show-all" to='/groups/browse/member'>{intl.formatMessage(messages.show_all)}</Link>}
                    </div>
                </div>
            </div>
        }
    }
}
