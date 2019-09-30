use yew::prelude::*;

use coolcats_ui_shared::router::RouteService;
use crate::application::{
    Action,
    context::{self, ContextAgent},
    state::State,
    interfaces::{
        settings::Settings, new_meow::NewMeow, find_meow::FindMeow, following_feed::FollowingFeed,
        hashtag_feed::HashtagFeed, user_feed::UserFeed,
    },
};

const DEFAULT_PROFILE_PIC: &str = "cat-eating-bird-circle.png";

interface_getstates!("app_properties", "first_name", "handle", "profile_pic");
interface_component!(App);
interface_view_only!(App);

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        if self.getstate.is_empty() {
            return html! {};
        };
        let app_properties = self.getstate.get_dict("app_properties");
        let first_name = self.getstate.string("first_name");
        let handle = self.getstate.string("handle");
        let profile_pic = self.getstate.string("profile_pic");

        if app_properties.string("Agent_Handle").is_empty() {
            html! {
                <div class="modal_backdrop">
                    <div class="modal_style">
                        <div align="center">
                            <p class="h1">{ "Welcome to Coolcats!" }</p>
                        </div>
                        <Settings counter = self.counter/>
                    </div>
                </div>
            }
        } else {
            let route_service: RouteService<()> = RouteService::new();
            let (route, route_param) = route_service.get_route_and_param();

            html! {
                <div class="container">
                    <div class="spinner transition500"/>
                    <div class="error transition500"/>
                    <div class="row first">
                        <div class="fixed-area">
                            <div class="col-sm-2 contentcontainer">
                                <div class="logo">
                                    <img
                                        src={
                                            if !profile_pic.is_empty() { &profile_pic }
                                            else { DEFAULT_PROFILE_PIC }
                                        },
                                        alt="user-profile"
                                    />
                                    <div id="displayName">{first_name}</div>
                                    <a href="/#/editProfile",
                                        id="handle"
                                    >
                                        {"@"}{handle}
                                    </a>
                                </div>
                            </div>
                            <div class="col-sm-7">
                                <div class="contentcontainer">
                                    <a href="/#/follow",
                                        id="followButton",
                                        class="btn btn-default"
                                    >
                                        {"Follow People"}
                                    </a>
                                    <div id="banner">
                                        <a href="/">{"Coolcats (Clutter)"}</a>
                                        <div class="subtitle">{"can haz herd cats?"}</div>
                                    </div>
                                    <div id="content">
                                        {match route.as_str() {
                                            "/" => html! {<NewMeow counter = self.counter/>},
                                            "/meow" => html! {
                                                <FindMeow
                                                    params = (
                                                        self.counter,
                                                        route_param.to_string()
                                                    )
                                                />
                                            },
                                            "/tag" => html! {
                                                <HashtagFeed
                                                    params = (
                                                        self.counter,
                                                        route_param.to_string()
                                                    )
                                                />
                                            },
                                            _ => html! {},
                                        }}
                                    </div>
                                </div>
                            </div>
                            <div class="col-sm-3">
                                <div class="alphabox">
                                    <div id="about">
                                        <h2>{"What is Clutter?"}</h2>
                                          <p>
                                              <a
                                                href="https://en.wiktionary.org/wiki/clutter",
                                                target="blank"
                                              >
                                                  <em>{"clutter"}</em>
                                              </a>
                                              {" is a flock of cats."}
                                          </p>
                                          <p>
                                              <strong>{"Clutter"}</strong>
                                              {" is a fully decentralized alternative to Twitter."}
                                          </p>
                                          <p>{"Impossible to censor or control."}</p>
                                          <p>
                                              {"Join the mewvolution on "}
                                              <a href="http://holochain.org", target="blank">
                                                  {"holochain.org"}
                                              </a>{"."}
                                          </p>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="row">
                            <div class="contentcontainer", id="feedContent">
                                <div>
                                    {match route.as_str() {
                                        "/" => html! {<FollowingFeed counter = self.counter/>},
                                        "/u" => html! {
                                            <UserFeed
                                                params = (self.counter, route_param.to_string())
                                            />
                                        },
                                        _ => html! {},
                                    }}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
    }
}
