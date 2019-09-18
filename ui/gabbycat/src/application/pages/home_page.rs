use yew::prelude::*;

use crate::application::features::{
    compose::compose_form::ComposeForm,
    ui::{user_panel::UserPanel, promo_panel::PromoPanel, link_footer::LinkFooter},
};

pub struct HomePage;

impl Renderable<HomePage> for HomePage {
    fn view(&self) -> Html<Self> {
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
                                    //<Avatar account={account} size={46} />
                                </div>
                                <ComposeForm />//should_condense = {true}, auto_focus = {false} />
                            </div>

                            //{children}
                        </div>
                    </div>

                    <div class = "columns-area__panels__pane columns-area__panels__pane--right">
                        <div class = "columns-area__panels__pane__inner">
                            //<GroupSidebarPanel />
                            //<WhoToFollowPanel />
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

pub enum Msg {}

impl Component for HomePage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}
