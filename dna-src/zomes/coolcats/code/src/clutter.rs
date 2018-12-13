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
        cas::content::Address,
        hash::HashString,
        json::JsonString,
    },
};

use super::anchors::{
    anchor,
    anchor_exists,
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
            },

            links: [
                from!(
                    "%agent_id",
                    tag: "handle",

                    validation_package: || {
                        hdk::ValidationPackageDefinition::ChainFull
                    },

                    validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                ),
                from!(
                    "%agent_id",
                    tag: "directory",

                    validation_package: || {
                        hdk::ValidationPackageDefinition::ChainFull
                    },

                    validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                )
            ]
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

// incomplete
fn use_handle(handle: String) -> ZomeApiResult<HashString> {
    hdk::debug(format!("use_handle('{}')", handle))?;
    let result = hdk::get_links(&AGENT_ADDRESS, "handle")?;
    let handles = result.addresses();
    if handles.len() > 0 {
        return Err(ZomeApiError::ValidationFailed("HandleInUse".into()));
    }
    if anchor_exists("handle", &handle)? {
        return Err(ZomeApiError::ValidationFailed("HandleInUse".into()));
    }
    let handle_address = hdk::commit_entry(&Handle::entry(&handle)?)?;
    hdk::link_entries(&AGENT_ADDRESS, &handle_address, "handle")?;
    hdk::link_entries(&AGENT_ADDRESS, &handle_address, "directory")?;
    Ok(handle_address)
}

fn get_handle() -> ZomeApiResult<Vec<String>> {
    hdk::debug(format!("get_handle(): {}", AGENT_ADDRESS.to_string()))?;
    let links = hdk::get_links(&AGENT_ADDRESS, "handle")?;
    let handles = links.addresses();
    if handles.len() > 0 {
        if let Some(handle_entry) = hdk::get_entry(handles[0].to_owned())? {
            let anchor_address = Address::from(
                handle_entry.value().to_string().trim_matches('"')
            );
            if let Some(anchor_entry) = hdk::get_entry(anchor_address)? {
                let anchor: serde_json::Value = serde_json::from_str(
                    &anchor_entry.value().to_string()
                ).unwrap();
                let anchor_text = anchor["anchor_text"].to_string();
                return Ok([anchor_text.trim_matches('"').into()].to_vec());
            }
        }
    }
    Ok(["".into()].to_vec())
}
