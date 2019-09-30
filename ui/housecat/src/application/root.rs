use std::str::FromStr;
use std::time::Duration;
use std::sync::Mutex;
use strum::AsStaticRef;
use serde::{Serialize, Deserialize};
use stdweb::web::{window, Date};

use yew::{
    prelude::*,
    services::{ConsoleService, DialogService, IntervalService, Task},
};

use coolcats_ui_shared::{
    Dict, DictItem,
    holoclient::{ToHoloclient, ToApplication},
    router::{self, Router, Route},
};

use super::{
    context::{self, ContextAgent},
    state::State,
    interfaces::{app::App, edit_profile::EditProfile, follow::Follow},
};

#[derive(EnumString, IntoStaticStr)]
pub enum RouterTarget {
    #[strum(serialize = "/")]
    App,
    #[strum(serialize = "/editProfile")]
    EditProfile,
    #[strum(serialize = "/follow")]
    Follow,
    #[strum(serialize = "/meow")]
    Meow,
    #[strum(serialize = "/error")]
    Error,
}

pub struct Root {
    callback: Callback<ToHoloclient>,
    state: State,
    conductor: String,
    child: RouterTarget,
    router: Box<dyn Bridge<Router<()>>>,
    context: Box<dyn Bridge<ContextAgent>>,
    link: ComponentLink<Root>,
    interval: IntervalService,
    interval_job: Option<Box<dyn Task>>,
}

#[derive(PartialEq, Clone)]
pub struct Params(pub ToApplication);

#[derive(Serialize, Deserialize)]
pub enum Action {
    Redirect(String),
    GetReady,
    GetHandles,
    UseHandle(String),
    SetFirstName(String),
    SetProfilePic(String),
    Follow(String),
    Unfollow(String),
    GetFollowing(String),
    Post(String),
    GetPostsBy(Vec<String>),
    GetPostsWithHashtag(String),
    AddFavourite(String),
    RemoveFavourite(String),
}

#[derive(EnumString, AsStaticStr)]
pub enum Redux {
    GetConductor,
    UseHandle,
    AgentHandle,
    GetHandles,
    SetFirstName,
    GetFirstName,
    SetProfilePic,
    GetProfilePic,
    Follow,
    Unfollow,
    GetFollowing,
    Post,
    GetPostsBy,
    GetPostsWithHashtag,
    AddFavourite,
    RemoveFavourite,
    GetFavourites,
}

pub enum Msg {
    Callback(ToHoloclient),
    Route(Route<()>),
    ChangeRoute(RouterTarget),
    Action(Action),
    ContextMsg(context::Response),
}

impl From<ToHoloclient> for Msg {
    fn from(msg: ToHoloclient) -> Self {
        Msg::Callback(msg)
    }
}

impl From<Action> for Msg {
    fn from(action: Action) -> Self {
        Msg::Action(action)
    }
}

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub params: Params,
    #[props(required)]
    pub callback: Callback<ToHoloclient>,
}

impl Component for Root {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let router = Router::bridge(link.send_back(Msg::Route));
        let context = ContextAgent::bridge(link.send_back(Msg::ContextMsg));
        let mut root = Self {
            callback: props.callback,
            state: Default::default(),
            conductor: String::new(),
            child: RouterTarget::App,
            router,
            context,
            link,
            interval: IntervalService::new(),
            interval_job: None,
        };
        root.load_profile();
        root.context.send(context::Request::SetRoot);
        root
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Callback(msg) => {
                self.callback.emit(msg);
            }

            Msg::Route(route) => {
                self.child = RouterTarget::from_str(&route.route).unwrap_or(RouterTarget::Error);
                return true;
            }

            Msg::ChangeRoute(target) => {
                let route: Route<()> = Route::set("/#", target.into());
                self.router.send(router::Request::ChangeRoute(route));
            }

            Msg::ContextMsg(response) => {
                if let context::Response::Request(who, request) = response {
                    match *request {
                        context::Request::GetStates(keys) => {
                            let keys: Vec<_> = keys.iter().map(String::as_str).collect();
                            self.context.send(
                                (
                                    who,
                                    context::Response::GetStates(
                                        self.state.subset(keys.as_slice()),
                                    ),
                                )
                                    .into(),
                            );
                        }

                        context::Request::Action(action) => {
                            self.update(Msg::Action(action));
                        }

                        _ => (),
                    }
                }
            }

            Msg::Action(action) => match action {
                Action::Redirect(path) => {
                    if path.as_str() == "/#/" {
                        self.update(Msg::ChangeRoute(RouterTarget::App));
                    }
                }

                Action::GetReady => {
                    self.state.set_bool("connected".into(), true);
                    self.get_my_handle();
                    self.get_handles();
                    self.get_profile_pic();
                    self.get_first_name();
                    self.get_favourites();
                    if self.interval_job.is_none() {
                        let send_msg = self.link.send_back(|_| Action::GetHandles.into());
                        let handle = self.interval.spawn(Duration::from_secs(2), send_msg);
                        self.interval_job = Some(Box::new(handle));
                    }
                }

                Action::GetHandles => {
                    self.get_handles();
                }

                Action::UseHandle(handle) => {
                    self.coolcats(
                        "use_handle",
                        &[("handle".into(), handle.into())],
                        Redux::UseHandle.as_static(),
                    );
                }

                Action::SetFirstName(first_name) => {
                    self.state.set_string("first_name".into(), first_name.clone());
                    self.coolcats(
                        "set_first_name",
                        &[("name".into(), first_name.into())],
                        Redux::SetFirstName.as_static(),
                    );
                    self.save_profile();
                    return true;
                }

                Action::SetProfilePic(profile_pic) => {
                    self.state.set_string("profile_pic".into(), profile_pic.clone());
                    self.coolcats(
                        "set_profile_pic",
                        &[("dataurl".into(), profile_pic.into())],
                        Redux::SetProfilePic.as_static(),
                    );
                    self.save_profile();
                    return true;
                }

                Action::Post(message) => {
                    let stamp = Date::now().to_string();
                    let mut post = Dict::new();
                    post.insert("author".into(), self.state.string("handle").clone().into());
                    post.insert("message".into(), message.clone().into());
                    self.state.mut_dict("posts").insert(stamp.clone(), post.into());
                    self.coolcats_meta(
                        "post",
                        &[
                            ("message".into(), message.into()),
                            ("stamp".into(), stamp.clone().into()),
                        ],
                        Redux::Post.as_static(),
                        &stamp,
                    );
                }

                Action::GetPostsBy(handles) => self.coolcats(
                    "get_posts_by",
                    &[("handles".into(), handles.into())],
                    Redux::GetPostsBy.as_static(),
                ),

                Action::GetPostsWithHashtag(hashtag) => self.coolcats(
                    "get_posts_with_hashtag",
                    &[("hashtag".into(), hashtag.into())],
                    Redux::GetPostsWithHashtag.as_static(),
                ),

                Action::Follow(user_handle) => self.coolcats_meta(
                    "follow",
                    &[("user_handle".into(), user_handle.clone().into())],
                    Redux::Follow.as_static(),
                    &user_handle,
                ),

                Action::Unfollow(user_handle) => self.coolcats_meta(
                    "unfollow",
                    &[("user_handle".into(), user_handle.clone().into())],
                    Redux::Unfollow.as_static(),
                    &user_handle,
                ),

                Action::GetFollowing(user_handle) => {
                    self.get_following(&user_handle);
                }

                Action::AddFavourite(address) => self.coolcats(
                    "add_favourite",
                    &[("address".into(), address.into())],
                    Redux::AddFavourite.as_static(),
                ),

                Action::RemoveFavourite(address) => self.coolcats(
                    "remove_favourite",
                    &[("address".into(), address.into())],
                    Redux::RemoveFavourite.as_static(),
                ),
            },
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let mut dialog = DialogService::new();
        let holoclient_msg = props.params.0;
        match holoclient_msg {
            ToApplication::Initialize => {
                self.update(
                    ToHoloclient::Call(("info/instances", Redux::GetConductor.as_static()).into())
                        .into(),
                );
            }

            ToApplication::Redux(result, redux, meta) => {
                let result = &json::parse(&result).expect("ToApplication::Redux: No result found");
                let redux = Redux::from_str(&redux).expect("ToApplication::Redux: No match");
                let value = &result["value"];

                match redux {
                    Redux::GetConductor => {
                        self.conductor = result[0]["id"].to_string();
                        self.update(Action::GetReady.into());
                    }

                    Redux::UseHandle => {
                        if value.is_null() {
                            let error = &result["error"];
                            if error["ValidationFailed"] == "handle_in_use" {
                                self.state.set_bool("handle_taken".into(), true);
                                return true;
                            }
                        } else {
                            let me = self.state.string("me").clone();
                            self.state.mut_dict("handles").set_string(value.to_string(), me);
                            self.state.set_bool("handle_taken".into(), false);
                            self.get_my_handle();
                        }
                    }

                    Redux::AgentHandle => {
                        if !value.is_null() {
                            let handle = value.to_string();
                            self.get_following(&handle);
                            if handle != *self.state.string("handle") {
                                self.state.set_string("handle".into(), handle.clone());
                                self.state
                                    .mut_dict("app_properties")
                                    .set_string("Agent_Handle".into(), handle);
                                self.save_profile();
                                return true;
                            }
                        } else if !self.state.string("handle").is_empty() {
                            self.state.set_string("handle".into(), "".into());
                            self.state
                                .mut_dict("app_properties")
                                .set_string("Agent_Handle".into(), "".into());
                            self.save_profile();
                            return true;
                        }
                    }

                    Redux::GetHandles => {
                        if !value.is_null() {
                            let mut changed = false;
                            let mut elem_num = 0;
                            while !value[elem_num].is_null() {
                                let element = &value[elem_num];
                                let address = element["address"].to_string();
                                let handle = element["handle"].to_string();
                                let handles = self.state.mut_dict("handles");
                                if handle != *handles.string(&address) {
                                    handles.insert(address, handle.into());
                                    changed = true;
                                }
                                elem_num += 1;
                            }
                            return changed;
                        }
                    }

                    Redux::SetFirstName | Redux::GetFirstName => {
                        if !value.is_null() {
                            let first_name = value.to_string();
                            if first_name != *self.state.string("first_name") {
                                self.state.set_string("first_name".into(), first_name);
                                self.save_profile();
                                return true;
                            }
                        } else if !self.state.string("first_name").is_empty() {
                            self.state.set_string("first_name".into(), "".into());
                            self.save_profile();
                            return true;
                        }
                    }

                    Redux::SetProfilePic | Redux::GetProfilePic => {
                        if !value.is_null() {
                            let profile_pic = value.to_string();
                            if profile_pic != *self.state.string("profile_pic") {
                                self.state.set_string("profile_pic".into(), profile_pic);
                                self.save_profile();
                                return true;
                            }
                        } else if !self.state.string("profile_pic").is_empty() {
                            self.state.set_string("profile_pic".into(), "".into());
                            self.save_profile();
                            return true;
                        }
                    }

                    Redux::Follow => {
                        let user_handle = meta;
                        if !value.is_null() && value.as_bool().unwrap() {
                            let follows = self.state.mut_dict("follows");
                            follows.insert(user_handle, true.into());
                            return true;
                        }
                    }

                    Redux::Unfollow => {
                        let user_handle = meta;
                        if !value.is_null() && value.as_bool().unwrap() {
                            let follows = self.state.mut_dict("follows");
                            follows.remove(&user_handle);
                            return true;
                        }
                    }

                    Redux::GetFollowing => {
                        if meta != *self.state.string("handle") {
                            dialog.alert(&format!(
                                "Redux::GetFollowing on handle {} not supported",
                                meta
                            ));
                            return false;
                        }
                        let follows = self.state.mut_dict("follows");
                        let mut i = 0;
                        while !value[i].is_null() {
                            let user_handle = value[i].to_string();
                            follows.insert(user_handle, true.into());
                            i += 1;
                        }
                        if i > 0 {
                            return true;
                        }
                    }

                    Redux::Post => {
                        let stamp = meta;
                        if !value.is_null() {
                            let address = value.to_string();
                            let post = self.state.mut_dict("posts").mut_dict(&stamp);
                            post.insert("address".into(), address.into());
                            return true;
                        } else {
                            self.state.mut_dict("posts").remove(&stamp);
                            dialog.alert("Redux::Post error");
                        }
                    }

                    Redux::GetPostsBy | Redux::GetPostsWithHashtag => {
                        let posts = self.state.mut_dict("posts");
                        let mut updated = false;
                        let mut i = 0;
                        while !value[i].is_null() {
                            let item = &value[i];
                            let post = &item["post"];
                            let stamp = post["stamp"].to_string();
                            if posts.get_dict(&stamp).is_empty() {
                                let address = item["address"].to_string();
                                let author = item["author"].to_string();
                                let message = post["message"].to_string();

                                let mut new_post = Dict::new();
                                new_post.insert("address".into(), address.into());
                                new_post.insert("author".into(), author.into());
                                new_post.insert("message".into(), message.into());
                                posts.insert(stamp, new_post.into());
                                updated = true;
                            }
                            i += 1;
                        }
                        return updated;
                    }

                    Redux::AddFavourite | Redux::RemoveFavourite | Redux::GetFavourites => {
                        let mut favourites: Vec<String> = Vec::new();
                        let mut i = 0;
                        while !value[i].is_null() {
                            let fave_address = value[i].to_string();
                            favourites.push(fave_address);
                            i += 1;
                        }
                        if favourites.len() != self.state.strings("favourites").len() {
                            self.state.set_strings("favourites".into(), favourites);
                            return true;
                        }
                    }
                }
            }

            ToApplication::None => (),
        }
        false
    }
}

impl Root {
    fn save_profile(&self) {
        let substate =
            self.state.subset(&["app_properties", "handle", "first_name", "profile_pic"]);
        window()
            .local_storage()
            .insert("coolcats_state", &serde_json::to_string(&substate).unwrap())
            .unwrap();
    }

    fn load_profile(&mut self) {
        if let Some(state) = window().local_storage().get("coolcats_state") {
            let substate: State = serde_json::from_str(&state).unwrap();
            self.state.merge(&substate);
        }
    }

    fn coolcats_meta(&mut self, method: &str, params: &[DictItem], redux: &str, meta: &str) {
        let call = ToHoloclient::Call(
            (&[self.conductor.as_str(), "coolcats", method][..], params, redux, meta).into(),
        );
        self.update(call.into());
    }

    fn coolcats(&mut self, method: &str, params: &[DictItem], redux: &str) {
        let call = ToHoloclient::Call(
            (&[self.conductor.as_str(), "coolcats", method][..], params, redux).into(),
        );
        self.update(call.into());
    }

    fn get_my_handle(&mut self) {
        self.coolcats(
            "app_property",
            &[("key".into(), "Agent_Handle".into())],
            Redux::AgentHandle.as_static(),
        );
    }

    fn get_handles(&mut self) {
        self.coolcats("get_handles", &[], Redux::GetHandles.as_static());
    }

    fn get_first_name(&mut self) {
        self.coolcats("get_first_name", &[], Redux::GetFirstName.as_static());
    }

    fn get_profile_pic(&mut self) {
        self.coolcats("get_profile_pic", &[], Redux::GetProfilePic.as_static());
    }

    fn get_following(&mut self, user_handle: &str) {
        self.coolcats_meta(
            "get_following",
            &[("user_handle".into(), user_handle.into())],
            Redux::GetFollowing.as_static(),
            user_handle,
        );
    }

    fn get_favourites(&mut self) {
        self.coolcats("get_favourites", &[], Redux::GetFavourites.as_static());
    }
}

impl RouterTarget {
    fn counter(&self) -> u32 {
        lazy_static! {
            static ref COUNTER: Mutex<u32> = Mutex::new(0);
        }
        let mut counter = COUNTER.lock().unwrap();
        *counter += 1;
        *counter
    }
}

impl Renderable<Root> for Root {
    fn view(&self) -> Html<Self> {
        let mut console = ConsoleService::new();
        console.log(&format!("state: {:#?}", self.state));
        self.child.view()
    }
}

impl Renderable<Root> for RouterTarget {
    fn view(&self) -> Html<Root> {
        // Send counter parameter to notify interface components of state changes
        let counter = self.counter();
        match self {
            RouterTarget::EditProfile => html! { <EditProfile counter = counter/> },
            RouterTarget::Follow => html! { <Follow counter = counter/> },
            _ => html! { <App counter = counter/> },
        }
    }
}
