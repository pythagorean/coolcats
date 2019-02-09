#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

mod utils;
mod anchors;
mod clutter;

use hdk::holochain_core_types::{
    error::HolochainError,
    json::JsonString,
};

use serde::{
    Serialize,
    Deserialize,
};

use anchors::{
    Anchor, AnchorLink,
};

use clutter::{
    Handle, FirstName, ProfilePic,
};

define_zome! {
    entries: [
       Anchor::definition(), AnchorLink::definition(),
       Handle::definition(), FirstName::definition(), ProfilePic::definition()
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
        app_property: {
            inputs: |key: String|,
            outputs: |result: JsonString|,
            handler: clutter::handle_app_property
        }
        use_handle: {
            inputs: |handle: String|,
            outputs: |result: JsonString|,
            handler: clutter::handle_use_handle
        }
        get_handle: {
            inputs: |address: String|,
            outputs: |result: JsonString|,
            handler: clutter::handle_get_handle
        }
        set_first_name: {
            inputs: |name: String|,
            outputs: |result: JsonString|,
            handler: clutter::handle_set_first_name
        }
        get_first_name: {
            inputs: | |,
            outputs: |result: JsonString|,
            handler: clutter::handle_get_first_name
        }
        set_profile_pic: {
            inputs: |dataurl: String|,
            outputs: |result: JsonString|,
            handler: clutter::handle_set_profile_pic
        }
        get_profile_pic: {
            inputs: | |,
            outputs: |result: JsonString|,
            handler: clutter::handle_get_profile_pic
        }
        log_out: {
            inputs: | |,
            outputs: |result: JsonString|,
            handler: clutter::handle_log_out
        }
    ]

    capabilities: {
        public (Public) [
            create_anchor, anchor_exists, get_anchor, get_anchors, app_property, use_handle,
            get_handle, set_first_name, get_first_name, set_profile_pic, get_profile_pic, log_out
        ]
    }
}
