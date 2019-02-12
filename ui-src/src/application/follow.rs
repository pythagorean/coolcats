use yew::prelude::*;

use crate::application::{
    Action,
    context::{ self, ContextAgent },
    state::State,
};

// Declare what state keys will be used by this component
const GETSTATES: [&str; 1] = ["follows"];

interface_getstates!();
interface_view_only!(Follow);
interface_component!(Follow);

impl Renderable<Follow> for Follow {
    fn view(&self) -> Html<Self> {
        if self.getstate.is_empty() {
            return html! { <></> };
        };
        let disable = true;
        html! {
            <div class="panel panel-default",>
                <div class="close",>
                    <a href="/#/",>{"x"}</a>
                </div>
                <div class="panel-body",>
                    <div class="row",>
                        <h3>{"Following"}</h3>
                        <ul id="following",>
                            //{this.props.following.length === 0 && (
                                <li>{"You currently aren't following anyone."}</li>
                            //)}
                            //{this.props.following.length > 0 && (
                            {if disable {html! {<></>}} else {html! {
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
                            //)}
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
                            //{filteredNotFollowing.length === 0 && (
                                <li>{"There are no users that you aren't already following."}</li>
                            //)}
                            //{filteredNotFollowing.length > 0 && (
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
                            //}
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
