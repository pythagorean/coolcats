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
        error::HolochainError,
        hash::HashString,
        json::JsonString,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct StoreHandle {
    handle: String,
}

pub fn handle_definition() -> ValidatingEntryType {
    entry!(
        name: "handle",
        description: "A user handle for posting meows",
        sharing: Sharing::Public,
        native_type: StoreHandle,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_handle: StoreHandle, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
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
    hdk::debug(format!("use_handle('{}')", handle)).unwrap();
    hdk::debug(format!("AGENT_ADDRESS = {}", AGENT_ADDRESS.to_string())).unwrap();
    let result = hdk::get_links(&AGENT_ADDRESS, "handle")?;
    let handles = result.addresses();
    if handles.len() > 0 {
        return Err(ZomeApiError::ValidationFailed("HandleInUse".into()));
    }
    if anchor_exists("handle", &handle) {
        return Err(ZomeApiError::ValidationFailed("HandleInUse".into()));
    }
    let entry = Entry::new(EntryType::App("handle".into()), StoreHandle { handle });
    hdk::commit_entry(&entry)
}

fn get_handle() -> ZomeApiResult<Vec<String>> {
    Ok(["".into()].to_vec())
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct StoreAnchor {
    anchor_type: String,
    anchor_text: String,
}

pub fn anchor_definition() -> ValidatingEntryType {
    entry!(
        name: "anchor",
        description: "An anchor type",
        sharing: Sharing::Public,
        native_type: StoreAnchor,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_anchor: StoreAnchor, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

fn anchor_exists(anchor_type: &str, anchor_text: &str) -> bool {
    let entry = Entry::new(EntryType::App("anchor".into()), StoreAnchor {
        anchor_type: anchor_type.into(),
        anchor_text: anchor_text.into(),
    });
    match hdk::get_entry(
        hdk::entry_address(&entry).unwrap()
    ).unwrap() {
        Some(_value) => true,
        None => false
    }
}
