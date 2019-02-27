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
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::utils::hdk_address_exists;

const ANCHOR_TYPES: &str = "anchor_types";

pub const ANCHOR: &str = "anchor";
#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Anchor {
    anchor_type: String,
    anchor_text: String,
}

const ANCHOR_LINK: &str = "anchor_link";
pub struct AnchorLink;

impl Anchor {
    pub fn definition() -> ValidatingEntryType {
        entry!(
            name: ANCHOR,
            description: "An anchor type",
            sharing: Sharing::Public,
            native_type: Anchor,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: |_anchor: Anchor, _ctx: hdk::ValidationData| {
                Ok(())
            }
        )
    }

    pub fn new(anchor_type: &str, anchor_text: &str) -> Self {
        Anchor {
            anchor_type: anchor_type.into(),
            anchor_text: anchor_text.into(),
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App(ANCHOR.into(), self.into())
    }

    pub fn create(anchor_type: &str, anchor_text: &str) -> ZomeApiResult<Address> {
        create_anchor(&Anchor::new(anchor_type, anchor_text))
    }

    pub fn unlink(anchor_type: &str, anchor_text: &str) -> ZomeApiResult<bool> {
        unlink_anchor(&Anchor::new(anchor_type, anchor_text))
    }

    pub fn address(anchor_type: &str, anchor_text: &str) -> ZomeApiResult<Address> {
        anchor_address(&Anchor::new(anchor_type, anchor_text))
    }

    pub fn exists(anchor_type: &str, anchor_text: &str) -> ZomeApiResult<bool> {
        anchor_exists(&Anchor::new(anchor_type, anchor_text))
    }

    pub fn list(anchor_type: &str) -> ZomeApiResult<Vec<Anchor>> {
        get_anchors(anchor_type)
    }

    pub fn get(addr: &Address) -> ZomeApiResult<Anchor> {
        get_anchor(addr)
    }

    pub fn get_type(&self) -> &String {
        &self.anchor_type
    }

    pub fn get_text(&self) -> &String {
        &self.anchor_text
    }
}

impl AnchorLink {
    pub fn definition() -> ValidatingEntryType {
        entry!(
            name: ANCHOR_LINK,
            description: "An anchor link type",
            sharing: Sharing::Public,
            native_type: Address,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: |_anchor_link: Address, _ctx: hdk::ValidationData| {
                Ok(())
            },

            links: [
                AnchorLink::anchor_link_definition()
            ]
        )
    }

    fn anchor_link_definition() -> ValidatingLinkDefinition {
        from!(
            ANCHOR,
            tag: ANCHOR_LINK,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
                Ok(())
            }
        )
    }

    fn create(anchor_addr: &Address) -> ZomeApiResult<Address> {
        hdk::commit_entry(&Entry::App(ANCHOR_LINK.into(), anchor_addr.into()))
    }
}

pub fn handle_create_anchor(anchor: Anchor) -> JsonString {
    match create_anchor(&anchor) {
        Ok(value) => json!({ "value": value }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_anchor_exists(anchor: Anchor) -> JsonString {
    match anchor_exists(&anchor) {
        Ok(value) => json!({ "value": value }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_anchor(address: String) -> JsonString {
    match get_anchor(&Address::from(address)) {
        Ok(value) => json!({ "value": value }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_anchors(anchor_type: String) -> JsonString {
    match get_anchors(&anchor_type) {
        Ok(value) => json!({ "value": value }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

fn create_anchor(anchor: &Anchor) -> ZomeApiResult<Address> {
    let anchor_entry = anchor.entry();
    let anchor_addr = hdk::entry_address(&anchor_entry)?;
    if hdk_address_exists(&anchor_addr)? {
        return Ok(anchor_addr);
    }
    hdk::commit_entry(&anchor_entry)?;
    let anchor_type_entry = Anchor::new(&anchor.anchor_type, "").entry();
    let anchor_type_addr = hdk::entry_address(&anchor_type_entry)?;
    if !hdk_address_exists(&anchor_type_addr)? {
        hdk::commit_entry(&anchor_type_entry)?;
        let root_anchor_type_entry = Anchor::new(ANCHOR_TYPES, "").entry();
        let root_anchor_type_addr = hdk::commit_entry(&root_anchor_type_entry)?;
        let anchor_type_link_addr = AnchorLink::create(&anchor_type_addr)?;
        hdk::link_entries(&root_anchor_type_addr, &anchor_type_link_addr, ANCHOR_LINK)?;
    }
    let anchor_link_addr = AnchorLink::create(&anchor_addr)?;
    hdk::link_entries(&anchor_type_addr, &anchor_link_addr, ANCHOR_LINK)?;
    Ok(anchor_addr)
}

fn unlink_anchor(anchor: &Anchor) -> ZomeApiResult<bool> {
    let anchor_entry = anchor.entry();
    let anchor_addr = hdk::entry_address(&anchor_entry)?;
    if !hdk_address_exists(&anchor_addr)? {
        return Ok(false);
    }
    let anchor_type_entry = Anchor::new(&anchor.anchor_type, "").entry();
    let anchor_type_addr = hdk::entry_address(&anchor_type_entry)?;
    let anchor_link_addr = AnchorLink::create(&anchor_addr)?;
    hdk::remove_link(&anchor_type_addr, &anchor_link_addr, ANCHOR_LINK)?;
    Ok(true)
}

fn anchor_address(anchor: &Anchor) -> ZomeApiResult<Address> {
    let anchor_entry = anchor.entry();
    hdk::entry_address(&anchor_entry)
}

fn anchor_exists(anchor: &Anchor) -> ZomeApiResult<bool> {
    let anchor_addr = anchor_address(anchor)?;
    Ok(hdk_address_exists(&anchor_addr)?)
}

fn get_anchor(addr: &Address) -> ZomeApiResult<Anchor> {
    if let Some(entry) = hdk::get_entry(addr)? {
        if let Entry::App(entry_type, value) = entry {
            match entry_type.to_string().as_ref() {
                ANCHOR => return Ok(Anchor::try_from(value)?),
                ANCHOR_LINK => {
                    let anchor_link = Address::try_from(value)?;
                    return get_anchor(&anchor_link);
                }
                _ => (),
            }
        }
    }
    Err(ZomeApiError::ValidationFailed("get_anchor called on non-anchor address".into()))
}

fn get_anchors(anchor_type: &str) -> ZomeApiResult<Vec<Anchor>> {
    let anchor_type_entry = Anchor::new(anchor_type, "").entry();
    let anchor_type_addr = hdk::entry_address(&anchor_type_entry)?;
    let anchor_type_links = hdk::get_links(&anchor_type_addr, ANCHOR_LINK)?;
    anchor_type_links.addresses().iter().map(|addr| get_anchor(&addr)).collect()
}
