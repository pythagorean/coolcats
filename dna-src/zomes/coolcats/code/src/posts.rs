use std::collections::HashSet;
use std::convert::TryFrom;

use hdk::{
    entry_definition::{
        ValidatingEntryType,
        ValidatingLinkDefinition,
    },
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
    holochain_wasm_utils::api_serialization::get_entry::{
        GetEntryOptions,
        GetEntryResultType
    },
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    utils::hdk_address_exists,
    anchors::{ANCHOR, Anchor},
    handles::{self, Handle},
};

pub const POST: &str = "post";
#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Post {
    message: String,
    stamp: String,
}

pub const HASHTAG: &str = "hashtag";

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
                if post.message.is_empty() {
                    return Err(String::from("Empty message"));
                }
                if post.message.len() > 255 {
                    return Err(String::from("Message too long"))
                }
                Ok(())
            },

            links: [
                Self::hashtag_link()
            ]
        )
    }

    fn hashtag_link() -> ValidatingLinkDefinition {
        from!(
            ANCHOR,
            tag: HASHTAG,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
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
        Ok(success) => json!({ "value": success }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_post(address: String) -> JsonString {
    match get_post(&address.into()) {
        Ok(success) => json!({ "value": success }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_posts_by(handles: Vec<String>) -> JsonString {
    match get_posts_by(handles.as_slice()) {
        Ok(success) => json!({ "value": success }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_posts_with_hashtag(hashtag: String) -> JsonString {
    match get_posts_with_hashtag(&hashtag) {
        Ok(success) => json!({ "value": success }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

fn post(message: &str, stamp: &str) -> ZomeApiResult<Address> {
    let post_addr = Post::create(message, stamp)?;
    let handle_addr = handles::get_handle_addr(None)?;
    hdk::link_entries(&handle_addr, &post_addr, POST)?;

    let hashtags = get_hashtags(message);
    for hashtag in hashtags {
        let anchor = Anchor::create(HASHTAG, &hashtag)?;
        hdk::link_entries(&anchor, &post_addr, HASHTAG)?;
    }

    Ok(post_addr)
}

#[derive(Serialize)]
pub struct GetPost {
    address: Address,
    post: Post,
    author: String,
}

impl GetPost {
    fn new(address: Address, post: Post, author: String) -> Self {
        GetPost {
            address,
            post,
            author,
        }
    }
}

fn get_post(addr: &Address) -> ZomeApiResult<GetPost> {
    if !hdk_address_exists(addr)? {
        return Err(ZomeApiError::ValidationFailed("post_not_found".into()));
    }
    if let GetEntryResultType::Single(result) = hdk::get_entry_result(
        addr,
        GetEntryOptions {
            entry: true,
            headers: true,
            ..Default::default()
        },
    )?
    .result
    {
        if let Entry::App(entry_type, value) = result.entry.unwrap() {
            if entry_type.to_string() == POST {
                let post = Post::try_from(value)?;
                let author = handles::get_handle(
                    &result
                        .headers
                        .into_iter()
                        .map(|header| header.provenances().first().unwrap().clone().source())
                        .next()
                        .unwrap(),
                )?;
                return Ok(GetPost::new(addr.clone(), post, author));
            }
        }
    }
    Err(ZomeApiError::ValidationFailed("post_not_found".into()))
}

fn get_posts_by(handles: &[String]) -> ZomeApiResult<Vec<GetPost>> {
    let mut posts: Vec<GetPost> = Vec::new();
    for user_handle in handles {
        let post_links = hdk::get_links(&Handle::address(user_handle)?, POST)?;
        for addr in post_links.addresses() {
            posts.push(get_post(&addr)?)
        }
    }
    Ok(posts)
}

fn get_hashtags(message: &str) -> Vec<String> {
    let mut hashtags: HashSet<String> = HashSet::new();
    for word in message.split_whitespace() {
        if word.len() > 1
            && word.starts_with('#')
            && word[1..].chars().skip_while(|c| c.is_alphabetic()).next().is_none()
        {
            hashtags.insert(word[1..].into());
        }
    }
    hashtags.into_iter().collect()
}

fn get_posts_with_hashtag(hashtag: &str) -> ZomeApiResult<Vec<GetPost>> {
    let hashtag = if hashtag.starts_with('#') {
        &hashtag[1..]
    } else {
        hashtag
    };
    let mut posts: Vec<GetPost> = Vec::new();
    let post_links = hdk::get_links(&Anchor::address(HASHTAG, &hashtag)?, HASHTAG)?;
    for addr in post_links.addresses() {
        posts.push(get_post(&addr)?);
    }
    Ok(posts)
}
