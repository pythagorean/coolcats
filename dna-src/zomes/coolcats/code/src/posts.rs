use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{
        cas::content::Address,
        entry::Entry,
        dna::entry_types::Sharing,
        error::HolochainError,
        json::JsonString,
    },
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::handles;

pub const POST: &str = "post";
#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Post {
    message: String,
    stamp: String,
}

impl Post {
    pub fn definition() -> ValidatingEntryType {
        entry!(
            name: POST,
            description: "A posted meow",
            sharing: Sharing::Public,
            native_type: Post,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: |_post: Post, _ctx: hdk::ValidationData| {
                Ok(())
            }
        )
    }

    fn new(message: &str, stamp: &str) -> Self {
        Post {
            message: message.into(),
            stamp: stamp.into(),
        }
    }

    fn entry(&self) -> Entry {
        Entry::App(POST.into(), self.into())
    }

    fn create(message: &str, stamp: &str) -> ZomeApiResult<Address> {
        hdk::commit_entry(&Post::new(message, stamp).entry())
    }
}

pub fn handle_post(message: String, stamp: String) -> JsonString {
    match post(&message, &stamp) {
        Ok(key) => json!({ "value": key }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

// incomplete
fn post(message: &str, stamp: &str) -> ZomeApiResult<Address> {
    let post_addr = Post::create(message, stamp)?;
    let handle_addr = handles::get_handle_addr(None)?;
    hdk::link_entries(&handle_addr, &post_addr, POST)?;
    // still needs to handle hashtags
    Ok(post_addr)
}
