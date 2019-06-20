use std::convert::TryFrom;

use hdk::{
    AGENT_ADDRESS,
    entry_definition::{
        ValidatingEntryType,
        ValidatingLinkDefinition,
    },
    error::ZomeApiResult,
    holochain_core_types::{
        cas::content::Address,
        entry::Entry,
        dna::entry_types::Sharing,
        error::HolochainError,
        json::JsonString,
        link::LinkMatch,
    },
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::utils::hdk_address_exists;

pub const FAVOURITE: &str = "favourite";
#[derive(Clone, Serialize, Deserialize, Debug, DefaultJson)]
pub struct Favourite(Address);

impl Favourite {
    pub fn definition() -> ValidatingEntryType {
        entry!(
            name: FAVOURITE,
            description: "A favourited address",
            sharing: Sharing::Public,

            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: |_validation_data: hdk::EntryValidationData<Favourite>| {
                Ok(())
            },

            links: [
                Self::link_from_agent()
            ]
        )
    }

    fn link_from_agent() -> ValidatingLinkDefinition {
        from!(
            "%agent_id",
            link_type: FAVOURITE,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::LinkValidationData | {
                Ok(())
            }
        )
    }

    fn new(fave_addr: &Address) -> Self {
        Favourite(fave_addr.clone())
    }

    fn entry(&self) -> Entry {
        Entry::App(FAVOURITE.into(), self.into())
    }

    fn create(fave_addr: &Address) -> ZomeApiResult<Address> {
        hdk::commit_entry(&Favourite::new(fave_addr).entry())
    }

    fn address(fave_addr: &Address) -> ZomeApiResult<Address> {
        hdk::entry_address(&Favourite::new(fave_addr).entry())
    }
}

pub fn handle_add_favourite(address: String) -> JsonString {
    match add_favourite(&address.into()) {
        Ok(success) => json!({ "value": success }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_remove_favourite(address: String) -> JsonString {
    match remove_favourite(&address.into()) {
        Ok(success) => json!({ "value": success }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_favourites() -> JsonString {
    match get_favourites() {
        Ok(success) => json!({ "value": success }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

fn add_favourite(fave_addr: &Address) -> ZomeApiResult<Vec<Address>> {
    if !hdk_address_exists(fave_addr)? {
        return Ok(Vec::new());
    }
    let mut faves = get_favourites()?;
    if !faves.iter().any(|x| *x == *fave_addr) {
        hdk::link_entries(&AGENT_ADDRESS, &Favourite::create(fave_addr)?, FAVOURITE, "")?;
        faves.push(fave_addr.clone());
    }
    Ok(faves)
}

fn remove_favourite(fave_addr: &Address) -> ZomeApiResult<Vec<Address>> {
    let mut found_fave = false;
    let faves: Vec<Address> = get_favourites()?
        .into_iter()
        .filter(|x| {
            if found_fave || *x != *fave_addr {
                true
            } else {
                found_fave = true;
                false
            }
        })
        .collect();
    if found_fave {
        hdk::remove_link(&AGENT_ADDRESS, &Favourite::address(fave_addr)?, FAVOURITE, "")?;
    }
    Ok(faves)
}

fn get_favourites() -> ZomeApiResult<Vec<Address>> {
    let mut faves: Vec<Address> = Vec::new();
    for entry in
        hdk::get_links_and_load(&AGENT_ADDRESS, LinkMatch::Exactly(FAVOURITE), LinkMatch::Any)?
    {
        if let Entry::App(entry_type, value) = entry? {
            if entry_type.to_string() == FAVOURITE {
                faves.push(Favourite::try_from(value)?.0);
            }
        }
    }
    Ok(faves)
}
