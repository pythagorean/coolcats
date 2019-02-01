use yew::prelude::*;
use yew_router::RouterAgent;

use crate::application::{
    Action,
    RouterTarget,
    state::State,
    settings::{ self, Settings },
    context::{ self, ContextAgent },
};

const DEFAULT_PROFILE_PIC: &str = "/cat-eating-bird-circle.png";

// Declare what state keys will be used by this component
const GETSTATES: [&str; 4] = ["app_properties", "first_name", "handle", "profile_pic"];

// Append state keys used by subcomponents
pub fn getstates() -> Vec<String> {
    let mut states = GETSTATES.to_vec();
    states.extend(settings::getstates());
    let states: Vec<_> = states.iter().map(|key| key.to_string()).collect();
    states
}

pub struct App {
    router: Box<Bridge<RouterAgent<()>>>,
    context: Box<Bridge<ContextAgent>>,
    getstate: State,
}

pub enum Msg {
    Action(Action),
    ChangeRoute(RouterTarget),
    ContextMsg(context::Response),
    GetStates,
    Ignore,
}

impl From<Action> for Msg {
    fn from(action: Action) -> Self {
        Msg::Action(action)
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let router = RouterAgent::bridge(link.send_back(|_| Msg::Ignore));
        let context = ContextAgent::bridge(link.send_back(Msg::ContextMsg));
        let mut app = Self {
            router,
            context,
            getstate: State::unset(),
        };
        app.update(Msg::GetStates);
        app
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Action(msg) => {
                js!{alert(@{format!("Msg::Action: {:?}", msg)})}
            }

            Msg::ChangeRoute(target) => {
                self.router.send(yew_router::Request::ChangeRoute(target.into()));
            }

            Msg::GetStates => {
                self.context.send(context::Request::GetStates(getstates()));
            }

            Msg::ContextMsg(response) => match response {
                context::Response::GetStates(getstate) => {
                    if self.getstate != getstate {
                        self.getstate = getstate;
                        return true;
                    }
                }

                context::Response::Request(_, _) => (),
            }

            Msg::Ignore => (),
        };
        false
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        if self.getstate.is_empty() { return html! { <></> }};
        let app_properties = self.getstate.get_dict("app_properties");
        let first_name = self.getstate.string("first_name");
        let handle = self.getstate.string("handle");
        let profile_pic = self.getstate.string("profile_pic");

        if app_properties.string("Agent_Handle").is_empty() {
            html! {
                <div class="modal_backdrop",>
                    <div class="modal_style",>
                        <div align="center",>
                            <p class="h1",>{ "Welcome to Coolcats2!" }</p>
                        </div>
                        <Settings:
                            getstate = self.getstate.subset(settings::getstates().as_slice()),
                            callback = Msg::Action,
                        />
                    </div>
                </div>
            }
        } else {
            html! {
                <div class="container",>
                    <div class="spinner transition500",/>
                    <div class="error transition500",/>
                    <div class="row first",>
                        <div class="fixed-area",>
                            <div class="col-sm-2 contentcontainer",>
                                <div class="logo",>
                                    <img
                                        src={
                                            if !profile_pic.is_empty() { &profile_pic }
                                            else { DEFAULT_PROFILE_PIC }
                                        },
                                        alt="user-profile",
                                    />
                                    <div id="displayName",>{first_name}</div>
                                    <a href="/editProfile", id="handle",>
                                        {"@"}{handle}
                                    </a>
                                </div>
                            </div>
                            <div class="col-sm-7",>
                                <div class="contentcontainer",>
                                    <a href="/follow",
                                        id="followButton",
                                        class="btn btn-default",
                                    >
                                        {"Follow People"}
                                    </a>
                                    <div id="banner",>
                                        <a href="/",>{"Coolcats2 (Clutter)"}</a>
                                        <div class="subtitle",>{"can haz herd cats?"}</div>
                                    </div>
                                    /*
                                    <div id="content",>
                                        <Route path="/" exact component={NewMeowContainer} />
                                        <Route path="/editProfile" component={EditProfileContainer} />
                                        <Route path="/follow" component={FollowContainer} />
                                        <Route path="/meow/:meowHash" component={MeowContainer} />
                                        <Route
                                            path="/tag/:hashtag"
                                            component={HashtagFeedContainer}
                                        />
                                    </div>
                                    */
                                </div>
                            </div>
                            <div class="col-sm-3",>
                                <div class="alphabox",>
                                    <div id="about",>
                                        <h2>{"What is Clutter?"}</h2>
                                          <p>
                                              <a
                                                href="https://en.wiktionary.org/wiki/clutter",
                                                target="blank",
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
                                              <a href="http://holochain.org", target="blank",>
                                                  {"holochain.org"}
                                              </a>{"."}
                                          </p>
                                          /*
                                          <form
                                            id="logout-form"
                                            onSubmit={this.onLogoutSubmit.bind(this)}
                                            action=""
                                          >
                                            <button
                                              type="submit"
                                              id="logout"
                                              className="btn btn-default btn-sm"
                                            >
                                              Logout
                                            </button>
                                          </form>
                                          */
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
    }
}
