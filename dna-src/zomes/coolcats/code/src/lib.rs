#![feature(try_from)]
#[macro_use]
extern crate hdk;
#[macro_use]
extern crate holochain_core_types_derive;

extern crate serde;
#[macro_use]
extern crate serde_json;

mod utils;
mod anchors;
mod handles;
mod props;
mod posts;

use hdk::holochain_core_types::{
    error::HolochainError,
    json::JsonString,
};

use serde::{Serialize, Deserialize};

use anchors::{Anchor, AnchorLink};
use handles::Handle;
use props::{FirstName, ProfilePic};
use posts::Post;

define_zome! {
    entries: [
       Anchor::definition(), AnchorLink::definition(), Handle::definition(),
       FirstName::definition(), ProfilePic::definition(), Post::definition()
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
        post: {
            inputs: |message: String, stamp: String|,
            outputs: |result: JsonString|,
            handler: posts::handle_post
        }
    ]

    traits: {
        hc_public [
            create_anchor, anchor_exists, get_anchor, get_anchors, use_handle, get_handle,
            app_property, set_first_name, get_first_name, set_profile_pic, get_profile_pic,
            post
        ]
    }
}
