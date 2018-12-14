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

use crate::anchors::{
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
                        hdk::ValidationPackageDefinition::Entry
                    },

                    validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    }

    fn entry(handle: &str) -> ZomeApiResult<Entry> {
        Ok(Entry::new(
            EntryType::App("handle".into()),
            anchor("handle", handle)?
        ))
    }
}

pub fn handle_use_handle(handle: String) -> JsonString {
    match use_handle(handle) {
        Ok(address) => json!({ "value": address }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_handle() -> JsonString {
    match get_handle() {
        Ok(handle) => json!({ "value": handle }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_log_out() -> JsonString {
    match log_out() {
        Ok(logged_out) => json!({ "value": logged_out }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

// incomplete
fn use_handle(handle: String) -> ZomeApiResult<HashString> {
    let links = hdk::get_links(&AGENT_ADDRESS, "handle")?;
    let addresses = links.addresses();
    if addresses.len() > 0 {
        return Err(ZomeApiError::ValidationFailed("handle_in_use".into()));
    }
    if anchor_exists("handle", &handle)? {
        return Err(ZomeApiError::ValidationFailed("handle_in_use".into()));
    }
    let handle_address = hdk::commit_entry(&Handle::entry(&handle)?)?;
    hdk::link_entries(&AGENT_ADDRESS, &handle_address, "handle")?;
    //hdk::link_entries(&AGENT_ADDRESS, &handle_address, "directory")?;
    Ok(handle_address)
}

fn get_handle() -> ZomeApiResult<String> {
    let links = hdk::get_links(&AGENT_ADDRESS, "handle")?;
    let addresses = links.addresses();
    if addresses.len() < 1 {
        return Ok("".into())
    }
    let handle_address = addresses[0].to_owned();
    if let Some(handle_entry) = hdk::get_entry(handle_address)? {
        let anchor_address = Address::from(
            handle_entry.value().to_string().trim_matches('"')
        );
        if let Some(anchor_entry) = hdk::get_entry(anchor_address)? {
            let anchor: serde_json::Value = serde_json::from_str(
                &anchor_entry.value().to_string()
            ).unwrap();
            let anchor_text = anchor["anchor_text"].to_string();
            return Ok(anchor_text.trim_matches('"').into())
        }
    }
    Ok("".into())
}

fn log_out() -> ZomeApiResult<String> {
    return Err(ZomeApiError::Internal("cannot delete link entries yet".into()));
}
