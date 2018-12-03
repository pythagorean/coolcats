use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::dna::zome::entry_types::Sharing,
    holochain_core_types::entry::{entry_type::EntryType, Entry},
    holochain_core_types::error::HolochainError,
    holochain_core_types::json::JsonString,
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
    let entry = Entry::new(EntryType::App("handle".into()), StoreHandle { handle });
    match hdk::commit_entry(&entry) {
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

fn get_handle() -> ZomeApiResult<Vec<String>> {
    Ok([String::from("")].to_vec())
}
