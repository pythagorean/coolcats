use yew::{prelude::*, agent::Dispatched};

use coolcats_ui_shared::router::{Router, Route, Request as RouterRequest};
use wildcat_macros::{StateComponent, UsesStateValues, use_state_values};
use crate::application::{
    context,
    state::State,
    routes::RouterTarget,
    facilities::avatar::Avatar,
    features::{
        compose::compose_form::ComposeForm,
        groups::sidebar_panel::GroupSidebarPanel,
        ui::{
            user_panel::UserPanel, promo_panel::PromoPanel, link_footer::LinkFooter,
            who_to_follow_panel::WhoToFollowPanel,
        },
    },
};

use_state_values!("app_properties");

#[derive(UsesStateValues, StateComponent)]
pub struct HomePage {
    context: Box<dyn Bridge<context::Worker>>,
    substate: State,
}

impl Renderable<HomePage> for HomePage {
    fn view(&self) -> Html<Self> {
        if !self.substate.is_set() {
            return html! {};
        }
        let app_properties = self.substate.get_dict("app_properties");

        if app_properties.string("Agent_Handle").is_empty() {
            let route: Route<()> = Route::set(RouterTarget::SettingsPage.into(), "");
            Router::dispatcher().send(RouterRequest::ChangeRoute(route));
            return html! {};
        }

        let account = "";
        html! {
            <div class = "page">
                <div class = "page__columns">
                    <div class = "columns-area__panels">

                        <div class = "columns-area__panels__pane columns-area__panels__pane--left">
                            <UserPanel />
                            <PromoPanel />
                            <LinkFooter />
                        </div>
                    </div>

                    <div class = "columns-area__panels__main">
                        <div class = "columns-area columns-area--mobile">
                            <div class = "timeline-compose-block">
                                <div class = "timeline-compose-block__avatar">
                                    <Avatar account = account, size = 46 />
                                </div>
                                <ComposeForm should_condense = true, auto_focus = false />
                            </div>

                            //{children}
                        </div>
                    </div>

                    <div class = "columns-area__panels__pane columns-area__panels__pane--right">
                        <div class = "columns-area__panels__pane__inner">
                            <GroupSidebarPanel />
                            <WhoToFollowPanel />
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
