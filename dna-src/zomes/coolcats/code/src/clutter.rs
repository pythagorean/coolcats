use std::convert::TryFrom;

use hdk::{
    self,
    AGENT_ADDRESS,
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
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::anchors::Anchor;

const HANDLE: &str = "handle";
pub struct Handle;

impl Handle {
    pub fn definition() -> ValidatingEntryType {
        entry!(
            name: HANDLE,
            description: "A user handle for posting meows",
            sharing: Sharing::Public,
            native_type: Address,

            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: |_handle_anchor: Address, _ctx: hdk::ValidationData| {
                Ok(())
            },

            links: [
                Handle::agent_link_definition()
            ]
        )
    }

    fn agent_link_definition() -> ValidatingLinkDefinition {
        from!(
            "%agent_id",
            tag: HANDLE,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
                Ok(())
            }
        )
    }

    fn create(handle: &str) -> ZomeApiResult<Address> {
        hdk::commit_entry(&Entry::App(HANDLE.into(), Anchor::create(HANDLE, handle)?.into()))
    }

    fn exists(handle: &str) -> ZomeApiResult<bool> {
        Anchor::exists(HANDLE, handle)
    }
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct PropValue(String);

impl PropValue {
    fn new(value: &str) -> Self {
        PropValue(value.into())
    }

    fn create(tag: &str, data: &str) -> ZomeApiResult<Address> {
        hdk::commit_entry(&Entry::App(tag.to_string().into(), PropValue::new(data).into()))
    }
}

const FIRST_NAME: &str = "first_name";
pub struct FirstName;

impl FirstName {
    pub fn definition() -> ValidatingEntryType {
        entry!(
            name: FIRST_NAME,
            description: "a user's first name",
            sharing: Sharing::Public,
            native_type: PropValue,

            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: |_name: PropValue, _ctx: hdk::ValidationData| {
                Ok(())
            },

            links: [
                FirstName::agent_link_definition()
            ]
        )
    }

    fn agent_link_definition() -> ValidatingLinkDefinition {
        from!(
            "%agent_id",
            tag: FIRST_NAME,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
                Ok(())
            }
        )
    }
}

pub fn handle_app_property(key: String) -> JsonString {
    match app_property(&key) {
        Ok(value) => json!({ "value": value }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_use_handle(handle: String) -> JsonString {
    match use_handle(&handle) {
        Ok(address) => json!({ "value": address }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_handle(address: String) -> JsonString {
    match get_handle(&address.into()) {
        Ok(handle) => json!({ "value": handle }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_set_first_name(name: String) -> JsonString {
    match set_first_name(&name) {
        Ok(name) => json!({ "value": name }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_first_name() -> JsonString {
    match get_first_name() {
        Ok(name) => json!({ "value": name }).into(),
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
fn app_property(key: &str) -> ZomeApiResult<String> {
    match key {
        "Agent_Address" => Ok(AGENT_ADDRESS.to_string()),
        "Agent_Handle" => get_handle(&AGENT_ADDRESS),
        _ => Err(ZomeApiError::ValidationFailed(format!("No App Property with key: {}", key))),
    }
}

// incomplete
pub fn use_handle(handle: &str) -> ZomeApiResult<Address> {
    if Handle::exists(handle)? {
        return Err(ZomeApiError::ValidationFailed("handle_in_use".into()));
    }
    let handle_addr = Handle::create(handle)?;
    hdk::link_entries(&AGENT_ADDRESS, &handle_addr, HANDLE)?;
    Anchor::create(&AGENT_ADDRESS.to_string(), HANDLE)?; // Store that link exists (KLUDGE)
    Ok(handle_addr)
}

fn get_handle(addr: &Address) -> ZomeApiResult<String> {
    if addr.to_string() == AGENT_ADDRESS.to_string() {
        if !Anchor::exists(&AGENT_ADDRESS.to_string(), HANDLE)? {
            return Err(ZomeApiError::ValidationFailed("handle_unset".into()));
        }
    }
    let mut addr = addr;
    let links = hdk::get_links(addr, HANDLE)?;
    let addrs = links.addresses();
    if !addrs.is_empty() {
        addr = &addrs[0];
    }
    if let Some(entry) = hdk::get_entry(&addr)? {
        if let Entry::App(entry_type, value) = entry {
            if entry_type.to_string() == HANDLE {
                return Ok(Anchor::get(&Address::try_from(value)?)?.get_text().to_string());
            }
        }
    }
    Err(ZomeApiError::ValidationFailed("get_handle called on non-handle address".into()))
}

fn set_first_name(name: &str) -> ZomeApiResult<String> {
    set_profile_prop(name, FIRST_NAME)
}

fn get_first_name() -> ZomeApiResult<String> {
    get_profile_prop(FIRST_NAME)
}

fn set_profile_prop(data: &str, tag: &str) -> ZomeApiResult<String> {
    let prop_addr = PropValue::create(tag, data)?;
    hdk::link_entries(&AGENT_ADDRESS, &prop_addr, tag)?;
    Anchor::create(&AGENT_ADDRESS.to_string(), tag)?; // Store that link exists (KLUDGE)
    Ok(data.to_string())
}

fn get_profile_prop(tag: &str) -> ZomeApiResult<String> {
    if !Anchor::exists(&AGENT_ADDRESS.to_string(), tag)? {
        return Err(ZomeApiError::ValidationFailed(format!("unlinked_tag: {}", tag).into()));
    }
    let links = hdk::get_links(&AGENT_ADDRESS, tag)?;
    let addrs = links.addresses();
    if addrs.is_empty() {
        return Err(ZomeApiError::ValidationFailed(format!("unlinked_tag: {}", tag).into()));
    }
    if let Some(entry) = hdk::get_entry(&addrs[0])? {
        if let Entry::App(entry_type, value) = entry {
            if entry_type.to_string() == tag {
                let data = PropValue::try_from(value)?;
                return Ok(data.0);
            }
        }
    }
    Err(ZomeApiError::Internal("linked entry mismatch in get_profile_prop".into()))
}

fn log_out() -> ZomeApiResult<String> {
    Err(ZomeApiError::Internal("cannot delete link entries yet".into()))
}
