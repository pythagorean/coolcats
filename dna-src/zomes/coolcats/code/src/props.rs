use std::convert::TryFrom;

use hdk::{
    DNA_ADDRESS,
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
        link::LinkMatch,
    },
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::handles;

#[derive(Clone, Serialize, Deserialize, Debug, DefaultJson)]
pub struct PropValue(String);

impl PropValue {
    fn new(value: &str) -> Self {
        PropValue(value.into())
    }

    fn create(prop: &str, data: &str) -> ZomeApiResult<Address> {
        hdk::commit_entry(&Entry::App(prop.to_string().into(), PropValue::new(data).into()))
    }
}

macro_rules! prop_definition {
    ($name:ident) => {
        pub fn definition() -> ValidatingEntryType {
            entry!(
                name: $name,
                description: "a user's first name",
                sharing: Sharing::Public,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::EntryValidationData<PropValue>| {
                    Ok(())
                },

                links: [
                    Self::agent_link_definition()
                ]
            )
        }

        fn agent_link_definition() -> ValidatingLinkDefinition {
            from!(
                "%agent_id",
                link_type: $name,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )
        }
    }
}

const FIRST_NAME: &str = "first_name";
pub struct FirstName;
impl FirstName {
    prop_definition!(FIRST_NAME);
}

const PROFILE_PIC: &str = "profile_pic";
pub struct ProfilePic;
impl ProfilePic {
    prop_definition!(PROFILE_PIC);
}

pub fn handle_app_property(key: String) -> JsonString {
    match app_property(&key) {
        Ok(value) => json!({ "value": value }).into(),
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

pub fn handle_set_profile_pic(dataurl: String) -> JsonString {
    match set_profile_pic(&dataurl) {
        Ok(data) => json!({ "value": data }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_profile_pic() -> JsonString {
    match get_profile_pic() {
        Ok(data) => json!({ "value": data }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

fn app_property(key: &str) -> ZomeApiResult<String> {
    match key {
        "DNA_Address" => Ok(DNA_ADDRESS.to_string()),
        "Agent_Address" => Ok(AGENT_ADDRESS.to_string()),
        "Agent_Handle" => handles::get_handle(&AGENT_ADDRESS),
        _ => Err(ZomeApiError::ValidationFailed(format!("No App Property with key: {}", key))),
    }
}

fn set_first_name(name: &str) -> ZomeApiResult<String> {
    set_profile_prop(FIRST_NAME, name)
}

fn get_first_name() -> ZomeApiResult<String> {
    get_profile_prop(FIRST_NAME)
}

fn set_profile_pic(dataurl: &str) -> ZomeApiResult<String> {
    set_profile_prop(PROFILE_PIC, dataurl)
}

fn get_profile_pic() -> ZomeApiResult<String> {
    get_profile_prop(PROFILE_PIC)
}

fn set_profile_prop(prop: &str, data: &str) -> ZomeApiResult<String> {
    let prop_addr = PropValue::create(prop, data)?;
    hdk::link_entries(&AGENT_ADDRESS, &prop_addr, prop, "")?;
    Ok(data.to_string())
}

fn get_profile_prop(prop: &str) -> ZomeApiResult<String> {
    let links = hdk::get_links(&AGENT_ADDRESS, LinkMatch::Exactly(prop), LinkMatch::Any)?;
    let addrs = links.addresses();
    if addrs.is_empty() {
        return Err(ZomeApiError::ValidationFailed(format!("unlinked_prop: {}", prop)));
    }
    if let Some(entry) = hdk::get_entry(&addrs.last().unwrap())? {
        if let Entry::App(entry_type, value) = entry {
            if entry_type.to_string() == prop {
                let data = PropValue::try_from(value)?;
                return Ok(data.0);
            }
        }
    }
    Err(ZomeApiError::Internal("linked entry mismatch in get_profile_prop".into()))
}
