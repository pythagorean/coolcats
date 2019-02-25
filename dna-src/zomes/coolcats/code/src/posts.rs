use std::convert::TryFrom;
use boolinator::Boolinator;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::{
        ZomeApiResult,
        ZomeApiError,
    },
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

use crate::handles::{self, Handle};

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

            validation: |post: Post, _ctx: hdk::ValidationData| {
                (post.message.len() > 0)
                    .ok_or_else(|| String::from("Empty message"))
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
        Ok(address) => json!({ "value": address }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_post(address: String) -> JsonString {
    match get_post(&address.into()) {
        Ok(post) => json!({ "value": post }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_posts_by(user_handle: String) -> JsonString {
    match get_posts_by(&user_handle) {
        Ok(posts) => json!({ "value": posts }).into(),
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

fn get_post(addr: &Address) -> ZomeApiResult<Post> {
    if let Some(entry) = hdk::get_entry(addr)? {
        if let Entry::App(entry_type, value) = entry {
            if entry_type.to_string() == POST {
                return Ok(Post::try_from(value)?)
            }
        }
    }
    Err(ZomeApiError::ValidationFailed("post_not_found".into()))
}

#[derive(Serialize)]
pub struct GetPostBy {
    address: Address,
    post: Post,
}

impl GetPostBy {
    fn new(address: Address, post: Post) -> Self {
        GetPostBy {
            address,
            post,
        }
    }
}

fn get_posts_by(user_handle: &str) -> ZomeApiResult<Vec<GetPostBy>> {
    let mut posts: Vec<GetPostBy> = Vec::new();
    let post_links = hdk::get_links(&Handle::address(user_handle)?, POST)?;
    for addr in post_links.addresses() {
        posts.push(GetPostBy::new(addr.clone(), get_post(&addr)?));
    }
    Ok(posts)
}
