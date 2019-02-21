use yew::prelude::*;

use crate::{
    utils::Dict,
    application::{
        Action,
        context::{ self, ContextAgent },
        state::State,
    },
};

interface_getstates!("handles", "handle", "follows");

interface_component!(Follow);

// This will be mapped to Follow.local:
pub struct Local {
    following: Vec<Dict>,
    not_following: Vec<Dict>,
    new_follow_text: String,
}

impl Local {
    fn new() -> Self {
        Self {
            following: Vec::new(),
            not_following: Vec::new(),
            new_follow_text: String::new(),
        }
    }
}

pub enum LocalMsg {
    NewStates,
}

impl Follow {
    fn local_update(&mut self, msg: LocalMsg) -> ShouldRender {
        match msg {
            LocalMsg::NewStates => {
                let handles = self.getstate.get_dict("handles");
                let my_handle = self.getstate.string("handle");
                let follows = self.getstate.get_dict("follows");

                self.local.following = follows
                    .raw()
                    .keys()
                    .map(|handle| {
                        let mut follow = Dict::new();
                        follow.insert("handle".into(), handle.clone().into());
                        follow
                    })
                    .collect();

                self.local.not_following = handles
                    .raw()
                    .keys()
                    .filter(|address| {
                        let user_handle = handles.string(address);
                        !follows.bool(user_handle).unwrap_or(false) && user_handle != my_handle
                    })
                    .map(|address| {
                        let user_handle = handles.string(address);
                        let mut no_follow = Dict::new();
                        no_follow.insert("address".into(), address.clone().into());
                        no_follow.insert("handle".into(), user_handle.clone().into());
                        no_follow
                    })
                    .collect();
            }
        }
        true
    }
}

impl Renderable<Follow> for Follow {
    fn view(&self) -> Html<Self> {
        let following = &self.local.following;
        let not_following = &self.local.not_following;
        let new_follow_text = &self.local.new_follow_text;
        let filtered_not_following: Vec<Dict> = not_following
            .iter()
            .filter(|unfollowed| {
                unfollowed
                    .string("handle")
                    .to_lowercase()
                    .starts_with(&new_follow_text.to_lowercase())
            })
            .cloned()
            .collect();

        html! {
            <div class="panel panel-default",>
                <div class="close",>
                    <a href="/#/",>{"x"}</a>
                </div>
                <div class="panel-body",>
                    <div class="row",>
                        <h3>{"Following"}</h3>
                        <ul id="following",>
                            {if following.is_empty() {html! {
                                <li>{"You currently aren't following anyone."}</li>
                            }} else {html! {
                                <div
                                    class="panel-body",
                                    style="overflow-y: scroll; height: 100px",
                                >
                                    <div
                                        class="mid-width wrapItems",
                                        style="\
                                            padding-top: 10px; \
                                            background-color: #eeeeee; \
                                            height: 100px;\
                                        ",
                                    >
                                        /*
                                        {this.props.following.map(user => {
                                          return (
                                            <li className="following-handle" key={user.handle}>
                                              <div className="col-xs-9">
                                                <span className="handle">{user.handle}</span>
                                              </div>
                                              <div
                                                className="col-xs-3"
                                                style={{ 'padding-bottom': '10px' }}
                                              >
                                                <button
                                                  type="button"
                                                  className="btn btn-default"
                                                  onClick={() => this.props.unfollow(user.handle)}
                                                >
                                                  Unfollow
                                                </button>
                                              </div>
                                            </li>
                                          )
                                        })}
                                        */
                                    </div>
                                </div>
                            }}}
                        </ul>
                    </div>

                    <div class="row",>
                        <h3 id="myModalLabel",>{"Follow someone"}</h3>
                        <div class="col-xs-12",>
                            <div class="form-group input-icon",>
                                <i>{"@"}</i>
                                /*
                                <input
                                    value={this.state.newFollowText}
                                    onChange={this.updateFollowText}
                                    type="text"
                                    className="form-control"
                                    id="followHandle"
                                    placeholder="handle"
                                />
                                */
                            </div>
                        </div>
                        <ul id="not-following",>
                            {if filtered_not_following.is_empty() {html! {
                                <li>{"There are no users that you aren't already following."}</li>
                            }} else {html! {
                                <div
                                    class="panel-body",
                                    style="overflow-y: scroll; height: 200px",
                                >
                                    <div
                                        class="mid-width wrapItems",
                                        style="\
                                            padding-top: 10px; \
                                            background-color: #eeeeee; \
                                            height: 200px;\
                                        ",
                                    >
                                        /*
                                        {filteredNotFollowing.map(user => {
                                          return (
                                            <li className="following-handle" key={user.handle}>
                                              <div className="col-xs-9">
                                                <span className="handle">{user.handle}</span>
                                              </div>
                                              <div
                                                className="col-xs-3"
                                                style={{ 'padding-bottom': '10px' }}
                                              >
                                                <button
                                                  type="button"
                                                  className="btn btn-default"
                                                  onClick={() => this.props.follow(user.handle)}
                                                >
                                                  Follow
                                                </button>
                                              </div>
                                            </li>
                                          )
                                        })}
                                        */
                                    </div>
                                </div>
                            }}}
                        </ul>
                        <div class="row",>
                            <div class="col-sm-1",/>
                            <div class="col-sm-4",/>
                            <div class="col-sm-6",>
                                /*
                                <button
                                  type="button"
                                  id="close"
                                  className="btn btn-primary pull-right"
                                  onClick={() => this.props.history.push('/')}
                                >
                                  Close
                                </button>
                                */
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
