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
    holochain_wasm_utils::api_serialization::get_entry::{
        GetEntryOptions,
        GetEntryResultType
    },
};

use crate::{
    utils::hdk_address_exists,
    anchors::Anchor,
    posts::POST,
};

pub const HANDLE: &str = "handle";
pub struct Handle;

pub const FOLLOWERS: &str = "followers";
pub const FOLLOWING: &str = "following";

impl Handle {
    pub fn definition() -> ValidatingEntryType {
        entry!(
            name: HANDLE,
            description: "A user handle for posting meows",
            sharing: Sharing::Public,

            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: |_validation_data: hdk::EntryValidationData<Address>| {
                Ok(())
            },

            links: [
                Self::link_from_agent(),
                Self::link_to_followers(),
                Self::link_to_following(),
                Self::link_to_post()
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
            validation: | _validation_data: hdk::LinkValidationData | {
                Ok(())
            }
        )
    }

    fn link_to_followers() -> ValidatingLinkDefinition {
        to!(
            HANDLE,
            tag: FOLLOWERS,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::LinkValidationData | {
                Ok(())
            }
        )
    }

    fn link_to_following() -> ValidatingLinkDefinition {
        to!(
            HANDLE,
            tag: FOLLOWING,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::LinkValidationData | {
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
            validation: | _validation_data: hdk::LinkValidationData | {
                Ok(())
            }
        )
    }

    fn entry(handle: &str) -> ZomeApiResult<Entry> {
        Ok(Entry::App(HANDLE.into(), Anchor::create(HANDLE, handle)?.into()))
    }

    fn create(handle: &str) -> ZomeApiResult<Address> {
        hdk::commit_entry(&Handle::entry(handle)?)
    }

    fn update(handle: &str, old_address: &Address) -> ZomeApiResult<Address> {
        hdk::update_entry(Handle::entry(handle)?, old_address)
    }

    pub fn address(handle: &str) -> ZomeApiResult<Address> {
        hdk::entry_address(&Handle::entry(handle)?)
    }

    fn exists(handle: &str) -> ZomeApiResult<bool> {
        hdk_address_exists(&Handle::address(handle)?)
    }

    fn list() -> ZomeApiResult<Vec<String>> {
        Ok(Anchor::list(HANDLE)?
            .iter()
            .map(|anchor| anchor.get_text().to_string())
            .filter(|handle| Handle::exists(handle).unwrap_or(false))
            .collect())
    }
}

pub fn handle_use_handle(handle: String) -> JsonString {
    match use_handle(&handle) {
        Ok(success) => json!({ "value": success }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_handle(address: String) -> JsonString {
    match get_handle(&address.into()) {
        Ok(success) => json!({ "value": success }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_agent(handle: String) -> JsonString {
    match get_agent(&handle) {
        Ok(success) => json!({ "value": success }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_handles() -> JsonString {
    match get_handles() {
        Ok(success) => json!({ "value": success }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_follow(user_handle: String) -> JsonString {
    match follow(&user_handle) {
        Ok(success) => json!({ "value": success }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_unfollow(user_handle: String) -> JsonString {
    match unfollow(&user_handle) {
        Ok(success) => json!({ "value": success }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_followers(user_handle: String) -> JsonString {
    match get_follow(&user_handle, FOLLOWERS) {
        Ok(success) => json!({ "value": success }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_following(user_handle: String) -> JsonString {
    match get_follow(&user_handle, FOLLOWING) {
        Ok(success) => json!({ "value": success }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

// does not implement directory_links
pub fn use_handle(handle: &str) -> ZomeApiResult<Address> {
    if Handle::exists(handle)? {
        return Err(ZomeApiError::ValidationFailed("handle_in_use".into()));
    }
    let links = hdk::get_links(&AGENT_ADDRESS, HANDLE)?;
    let addrs = links.addresses();
    if !addrs.is_empty() {
        let old_handle_addr = &addrs[0];
        let new_handle_addr = Handle::create(handle)?;
        Anchor::unlink(HANDLE, &get_handle(old_handle_addr)?)?;
        Handle::update(handle, old_handle_addr)?;
        hdk::remove_link(&AGENT_ADDRESS, old_handle_addr, HANDLE)?;
        hdk::link_entries(&AGENT_ADDRESS, &new_handle_addr, HANDLE)?;
        return Ok(new_handle_addr);
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

pub fn get_agent(handle: &str) -> ZomeApiResult<Address> {
    if !Handle::exists(handle)? {
        return Err(ZomeApiError::ValidationFailed("handle_not_found".into()));
    }
    if let GetEntryResultType::Single(result) = hdk::get_entry_result(
        &Handle::address(handle)?,
        GetEntryOptions {
            entry: false,
            headers: true,
            ..Default::default()
        },
    )?
    .result
    {
        let agent = result
            .headers
            .into_iter()
            .map(|header| header.provenances().first().unwrap().clone().source())
            .next()
            .unwrap();
        return Ok(agent);
    } else {
        unimplemented!()
    }
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

pub fn get_handles() -> ZomeApiResult<Vec<GetHandle>> {
    let mut handles: Vec<GetHandle> = Vec::new();
    for handle in Handle::list()? {
        let address = Handle::address(&handle)?;
        handles.push(GetHandle::new(address, handle));
    }
    Ok(handles)
}

pub fn follow(user_handle: &str) -> ZomeApiResult<bool> {
    let follow_addr = Handle::address(user_handle)?;
    let handle_addr = get_handle_addr(None)?;
    hdk::link_entries(&follow_addr, &handle_addr, FOLLOWERS)?;
    hdk::link_entries(&handle_addr, &follow_addr, FOLLOWING)?;
    Ok(true)
}

pub fn unfollow(user_handle: &str) -> ZomeApiResult<bool> {
    let follow_addr = Handle::address(user_handle)?;
    let handle_addr = get_handle_addr(None)?;
    hdk::remove_link(&follow_addr, &handle_addr, FOLLOWERS)?;
    hdk::remove_link(&handle_addr, &follow_addr, FOLLOWING)?;
    Ok(true)
}

pub fn get_follow(user_handle: &str, tag: &str) -> ZomeApiResult<Vec<String>> {
    let user_addr = Handle::address(user_handle)?;
    let links = hdk::get_links(&user_addr, tag)?;
    let mut follow: Vec<String> = Vec::new();
    for follow_addr in links.addresses() {
        follow.push(get_handle(&follow_addr)?);
    }
    Ok(follow)
}
