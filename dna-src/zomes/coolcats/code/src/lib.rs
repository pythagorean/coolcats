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
mod links;
mod utils;

define_zome! {
    entries: [
		clutter::handle_definition(),
        anchors::anchor_definition(),
        links::links_definition()
	]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            get_handle: {
                inputs: | |,
                outputs: |result: JsonString|,
                handler: clutter::handle_get_handle
            }
            use_handle: {
                inputs: |handle: String|,
                outputs: |result: JsonString|,
                handler: clutter::handle_use_handle
            }
        }
    }
}
