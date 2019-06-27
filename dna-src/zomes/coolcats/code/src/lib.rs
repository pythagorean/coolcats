#![feature(try_from)]
#[macro_use]
extern crate hdk;
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

use hdk::holochain_json_api::{
    json::JsonString,
    error::JsonError,
};

use serde::{Serialize, Deserialize};

use anchors::Anchor;
use handles::Handle;
use props::{FirstName, ProfilePic};
use posts::Post;
use favourites::Favourite;

define_zome! {
    entries: [
       Anchor::definition(), Handle::definition(), FirstName::definition(),
       ProfilePic::definition(), Post::definition(), Favourite::definition()
    ]

    genesis: || { Ok(()) }

    functions: [
        create_anchor: {
            inputs: |anchor: Anchor|,
            outputs: |result: JsonString|,
            handler: anchors::handle_create_anchor
        }
        anchor_exists: {
            inputs: |anchor: Anchor|,
            outputs: |result: JsonString|,
            handler: anchors::handle_anchor_exists
        }
        get_anchor: {
            inputs: |address: String|,
            outputs: |result: JsonString|,
            handler: anchors::handle_get_anchor
        }
        get_anchors: {
            inputs: |anchor_type: String|,
            outputs: |result: JsonString|,
            handler: anchors::handle_get_anchors
        }
        use_handle: {
            inputs: |handle: String|,
            outputs: |result: JsonString|,
            handler: handles::handle_use_handle
        }
        get_handle: {
            inputs: |address: String|,
            outputs: |result: JsonString|,
            handler: handles::handle_get_handle
        }
        get_agent: {
            inputs: |handle: String|,
            outputs: |result: JsonString|,
            handler: handles::handle_get_agent
        }
        get_handles: {
            inputs: | |,
            outputs: |result: JsonString|,
            handler: handles::handle_get_handles
        }
        app_property: {
            inputs: |key: String|,
            outputs: |result: JsonString|,
            handler: props::handle_app_property
        }
        set_first_name: {
            inputs: |name: String|,
            outputs: |result: JsonString|,
            handler: props::handle_set_first_name
        }
        get_first_name: {
            inputs: | |,
            outputs: |result: JsonString|,
            handler: props::handle_get_first_name
        }
        set_profile_pic: {
            inputs: |dataurl: String|,
            outputs: |result: JsonString|,
            handler: props::handle_set_profile_pic
        }
        get_profile_pic: {
            inputs: | |,
            outputs: |result: JsonString|,
            handler: props::handle_get_profile_pic
        }
        follow: {
            inputs: |user_handle: String|,
            outputs: |result: JsonString|,
            handler: handles::handle_follow
        }
        unfollow: {
            inputs: |user_handle: String|,
            outputs: |result: JsonString|,
            handler: handles::handle_unfollow
        }
        get_followers: {
            inputs: |user_handle: String|,
            outputs: |result: JsonString|,
            handler: handles::handle_get_followers
        }
        get_following: {
            inputs: |user_handle: String|,
            outputs: |result: JsonString|,
            handler: handles::handle_get_following
        }
        post: {
            inputs: |message: String, stamp: String|,
            outputs: |result: JsonString|,
            handler: posts::handle_post
        }
        get_post: {
            inputs: |address: String|,
            outputs: |result: JsonString|,
            handler: posts::handle_get_post
        }
        get_posts_by: {
            inputs: |handles: Vec<String>|,
            outputs: |result: JsonString|,
            handler: posts::handle_get_posts_by
        }
        get_posts_with_hashtag: {
            inputs: |hashtag: String|,
            outputs: |result: JsonString|,
            handler: posts::handle_get_posts_with_hashtag
        }
        add_favourite: {
            inputs: |address: String|,
            outputs: |result: JsonString|,
            handler: favourites::handle_add_favourite
        }
        remove_favourite: {
            inputs: |address: String|,
            outputs: |result: JsonString|,
            handler: favourites::handle_remove_favourite
        }
        get_favourites: {
            inputs: | |,
            outputs: |result: JsonString|,
            handler: favourites::handle_get_favourites
        }
    ]

    traits: {
        hc_public [
            create_anchor, anchor_exists, get_anchor, get_anchors,
            use_handle, get_handle, get_agent, get_handles,
            follow, unfollow, get_followers, get_following,
            app_property, set_first_name, get_first_name, set_profile_pic, get_profile_pic,
            post, get_post, get_posts_by, get_posts_with_hashtag,
            add_favourite, remove_favourite, get_favourites
        ]
    }
}
