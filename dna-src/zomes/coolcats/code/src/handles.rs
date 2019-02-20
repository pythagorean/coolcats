use std::convert::TryFrom;
use serde::Serialize;

use hdk::{
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
        json::JsonString,
    },
};

use crate::anchors::Anchor;
use crate::posts::POST;

pub const HANDLE: &str = "handle";
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
                Self::link_from_agent(), Self::link_to_post()
            ]
        )
    }

    fn link_from_agent() -> ValidatingLinkDefinition {
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

    fn link_to_post() -> ValidatingLinkDefinition {
        to!(
            POST,
            tag: POST,
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

    fn address(handle: &str) -> ZomeApiResult<Address> {
        Anchor::address(HANDLE, handle)
    }

    fn exists(handle: &str) -> ZomeApiResult<bool> {
        Anchor::exists(HANDLE, handle)
    }

    fn list() -> ZomeApiResult<Vec<String>> {
        let anchors = Anchor::list(HANDLE)?;
        Ok(anchors.iter().map(|anchor| anchor.get_text().to_string()).collect())
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

pub fn handle_get_handles() -> JsonString {
    match get_handles() {
        Ok(handles) => json!({ "value": handles }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

// incomplete
pub fn use_handle(handle: &str) -> ZomeApiResult<Address> {
    if Handle::exists(handle)? {
        return Err(ZomeApiError::ValidationFailed("handle_in_use".into()));
    }
    let handle_addr = Handle::create(handle)?;
    hdk::link_entries(&AGENT_ADDRESS, &handle_addr, HANDLE)?;
    Ok(handle_addr)
}

pub fn get_handle(addr: &Address) -> ZomeApiResult<String> {
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
    Err(ZomeApiError::ValidationFailed("handle_not_found".into()))
}

pub fn get_handle_addr(addr: Option<&Address>) -> ZomeApiResult<Address> {
    let mut addr = addr.unwrap_or(&AGENT_ADDRESS);
    let links = hdk::get_links(addr, HANDLE)?;
    let addrs = links.addresses();
    if !addrs.is_empty() {
        addr = &addrs[0];
    }
    if let Some(entry) = hdk::get_entry(&addr)? {
        if let Entry::App(entry_type, _) = entry {
            if entry_type.to_string() == HANDLE {
                return Ok(addr.clone());
            }
        }
    }
    Err(ZomeApiError::ValidationFailed("handle_not_found".into()))
}

#[derive(Serialize)]
pub struct GetHandle {
    address: Address,
    handle: String,
}

impl GetHandle {
    fn new(address: Address, handle: String) -> Self {
        GetHandle {
            address,
            handle,
        }
    }
}

// not very efficient or anything but should work for testing
pub fn get_handles() -> ZomeApiResult<Vec<GetHandle>> {
    let mut handles: Vec<GetHandle> = Vec::new();
    for handle in Handle::list()? {
        let address = Handle::address(&handle)?;
        handles.push(GetHandle::new(address, handle));
    }
    Ok(handles)
}
