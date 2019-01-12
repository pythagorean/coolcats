#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

mod clutter;
mod anchors;
mod utils;

define_zome! {
    entries: [
		clutter::Handle::definition(),
        anchors::Anchor::definition()
	]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            app_property: {
                inputs: |name: String|,
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
            log_out: {
                inputs: | |,
                outputs: |result: JsonString|,
                handler: clutter::handle_log_out
            }
        }
    }
}
