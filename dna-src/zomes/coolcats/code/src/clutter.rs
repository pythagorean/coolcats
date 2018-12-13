use hdk::{
    self,
    AGENT_ADDRESS,
    entry_definition::ValidatingEntryType,
    error::{
        ZomeApiResult,
        ZomeApiError,
    },
    holochain_core_types::{
        dna::zome::entry_types::Sharing,
        entry::{
            Entry,
            entry_type::EntryType,
        },
        hash::HashString,
        json::JsonString,
    },
};

use super::anchors::{
    anchor,
    anchor_exists,
};

use crate::links::{
    Link,
    Links
};

pub struct Handle {}

impl Handle {
    pub fn definition() -> ValidatingEntryType {
        entry!(
            name: "handle",
            description: "A user handle for posting meows",
            sharing: Sharing::Public,
            native_type: HashString,

            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: |_handle_anchor: HashString, _ctx: hdk::ValidationData| {
                Ok(())
            }
        )
    }

    pub fn entry(handle: &str) -> ZomeApiResult<Entry> {
        Ok(Entry::new(
            EntryType::App("handle".into()),
            anchor("handle", handle)?
        ))
    }
}

pub fn handle_use_handle(handle: String) -> JsonString {
    match use_handle(handle) {
        Ok(address) => json!({ "address": address }).into(),
        Err(hdk_err) => hdk_err.into(),
    }
}

pub fn handle_get_handle() -> JsonString {
    match get_handle() {
        Ok(result) => result.into(),
        Err(hdk_err) => hdk_err.into(),
    }
}

fn use_handle(handle: String) -> ZomeApiResult<HashString> {
    hdk::debug(format!("use_handle('{}')", handle))?;
    hdk::debug(format!("AGENT_ADDRESS = {}", AGENT_ADDRESS.to_string()))?;
    let result = hdk::get_links(&AGENT_ADDRESS, "handle")?;
    let handles = result.addresses();
    if handles.len() > 0 {
        return Err(ZomeApiError::ValidationFailed("HandleInUse".into()));
    }
    if anchor_exists("handle", &handle)? {
        return Err(ZomeApiError::ValidationFailed("HandleInUse".into()));
    }
    let handle_address = hdk::commit_entry(&Handle::entry(&handle)?)?;
    hdk::commit_entry(&Links::entry("handle_links",
        Link::new(&AGENT_ADDRESS, &handle_address, "handle")
    ))?;
    hdk::commit_entry(&Links::entry("directory_links",
        Link::new(&AGENT_ADDRESS, &handle_address, "directory")
    ))?;
    Ok(handle_address)
}

fn get_handle() -> ZomeApiResult<Vec<String>> {
    Ok(["".into()].to_vec())
}
