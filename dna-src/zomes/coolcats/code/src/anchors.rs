use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
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

use crate::utils::{
    address_exists,
    entry_exists
};

use crate::links::{
    Link,
    Links
};

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Anchor {
    anchor_type: String,
    anchor_text: String,
}

impl Anchor {
    pub fn definition() -> ValidatingEntryType {
        entry!(
            name: "anchor",
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

    pub fn new(anchor_type: &str, anchor_text: &str) -> Anchor {
        Anchor {
            anchor_type: anchor_type.to_owned(),
            anchor_text: anchor_text.to_owned(),
        }
    }

    pub fn entry(anchor_type: &str, anchor_text: &str) -> Entry {
        Entry::new(
            EntryType::App("anchor".into()),
            Anchor::new(anchor_type, anchor_text)
        )
    }
}

pub fn anchor(anchor_type: &str, anchor_text: &str) -> ZomeApiResult<HashString> {
    let anchor_entry = Anchor::entry(anchor_type, anchor_text);
    let anchor_address = hdk::entry_address(&anchor_entry)?;
    if address_exists(&anchor_address)? {
        return Ok(anchor_address);
    }
    let anchor_type_entry = Anchor::entry(anchor_type, "");
    let anchor_type_address = hdk::entry_address(&anchor_type_entry)?;
    if !address_exists(&anchor_type_address)? {
        let root_anchor_type_entry = Anchor::entry("anchor_types", "");
        let root_anchor_type_address = hdk::entry_address(&root_anchor_type_entry)?;
        if !address_exists(&root_anchor_type_address)? {
            hdk::commit_entry(&root_anchor_type_entry)?;
        }
        let anchor_type_links_entry = Links::entry("anchor_links",
            Link::new(&root_anchor_type_address, &anchor_type_address, &anchor_type)
        );
        hdk::commit_entry(&anchor_type_entry)?;
        hdk::commit_entry(&anchor_type_links_entry)?;
    }
    let anchor_links_entry = Links::entry("anchor_links",
        Link::new(&anchor_type_address, &anchor_address, &anchor_text)
    );
    hdk::commit_entry(&anchor_entry)?;
    hdk::commit_entry(&anchor_links_entry)?;
    Ok(anchor_address)
}

pub fn anchor_exists(anchor_type: &str, anchor_text: &str) -> ZomeApiResult<bool> {
    entry_exists(&Anchor::entry(anchor_type, anchor_text))
}
