#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
#[macro_use]
extern crate holochain_json_derive;

extern crate serde;
#[macro_use]
extern crate serde_json;

mod utils;
mod anchors;
mod handles;
mod props;
mod posts;
mod favourites;

use hdk::entry_definition::ValidatingEntryType;
use hdk_proc_macros::zome;

use serde::{Serialize, Deserialize};

use anchors::Anchor;
use handles::Handle;
use props::{FirstName, ProfilePic};
use posts::Post;
use favourites::Favourite;

#[zome]
pub mod main {
    #[genesis]
    pub fn genesis() {
        Ok(())
    }

    // Entry definitions:

    #[entry_def]
    fn anchor_entry_def() -> ValidatingEntryType {
        Anchor::definition()
    }

    #[entry_def]
    fn handle_entry_def() -> ValidatingEntryType {
        Handle::definition()
    }

    #[entry_def]
    fn first_name_def() -> ValidatingEntryType {
        FirstName::definition()
    }

    #[entry_def]
    fn profile_pic_def() -> ValidatingEntryType {
        ProfilePic::definition()
    }

    #[entry_def]
    fn post_def() -> ValidatingEntryType {
        Post::definition()
    }

    #[entry_def]
    fn favourite_def() -> ValidatingEntryType {
        Favourite::definition()
    }

    // Zome functions:

    #[zome_fn("hc_public")]
    fn create_anchor(anchor: Anchor) -> JsonString {
        anchors::handle_create_anchor(anchor)
    }

    #[zome_fn("hc_public")]
    fn anchor_exists(anchor: Anchor) -> JsonString {
        anchors::handle_anchor_exists(anchor)
    }

    #[zome_fn("hc_public")]
    fn get_anchor(address: String) -> JsonString {
        anchors::handle_get_anchor(address)
    }

    #[zome_fn("hc_public")]
    fn get_anchors(anchor_type: String) -> JsonString {
        anchors::handle_get_anchors(anchor_type)
    }

    #[zome_fn("hc_public")]
    fn use_handle(handle: String) -> JsonString {
        handles::handle_use_handle(handle)
    }

    #[zome_fn("hc_public")]
    fn get_handle(address: String) -> JsonString {
        handles::handle_get_handle(address)
    }

    #[zome_fn("hc_public")]
    fn get_agent(handle: String) -> JsonString {
        handles::handle_get_agent(handle)
    }

    #[zome_fn("hc_public")]
    fn get_handles() -> JsonString {
        handles::handle_get_handles()
    }

    #[zome_fn("hc_public")]
    fn app_property(key: String) -> JsonString {
        props::handle_app_property(key)
    }

    #[zome_fn("hc_public")]
    fn set_first_name(name: String) -> JsonString {
        props::handle_set_first_name(name)
    }

    #[zome_fn("hc_public")]
    fn get_first_name() -> JsonString {
        props::handle_get_first_name()
    }

    #[zome_fn("hc_public")]
    fn set_profile_pic(dataurl: String) -> JsonString {
        props::handle_set_profile_pic(dataurl)
    }

    #[zome_fn("hc_public")]
    fn get_profile_pic() -> JsonString {
        props::handle_get_profile_pic()
    }

    #[zome_fn("hc_public")]
    fn follow(user_handle: String) -> JsonString {
        handles::handle_follow(user_handle)
    }

    #[zome_fn("hc_public")]
    fn unfollow(user_handle: String) -> JsonString {
        handles::handle_unfollow(user_handle)
    }

    #[zome_fn("hc_public")]
    fn get_followers(user_handle: String) -> JsonString {
        handles::handle_get_followers(user_handle)
    }

    #[zome_fn("hc_public")]
    fn get_following(user_handle: String) -> JsonString {
        handles::handle_get_following(user_handle)
    }

    #[zome_fn("hc_public")]
    fn post(message: String, stamp: String) -> JsonString {
        posts::handle_post(message, stamp)
    }

    #[zome_fn("hc_public")]
    fn get_post(address: String) -> JsonString {
        posts::handle_get_post(address)
    }

    #[zome_fn("hc_public")]
    fn get_posts_by(handles: Vec<String>) -> JsonString {
        posts::handle_get_posts_by(handles)
    }

    #[zome_fn("hc_public")]
    fn get_posts_with_hashtag(hashtag: String) -> JsonString {
        posts::handle_get_posts_with_hashtag(hashtag)
    }

    #[zome_fn("hc_public")]
    fn add_favourite(address: String) -> JsonString {
        favourites::handle_add_favourite(address)
    }

    #[zome_fn("hc_public")]
    fn remove_favourite(address: String) -> JsonString {
        favourites::handle_remove_favourite(address)
    }

    #[zome_fn("hc_public")]
    fn get_favourites() -> JsonString {
        favourites::handle_get_favourites()
    }
}
