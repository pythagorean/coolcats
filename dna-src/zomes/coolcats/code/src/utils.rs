use hdk::{
    error::ZomeApiResult,
    holochain_core_types::cas::content::Address,
};

pub fn hdk_address_exists(addr: &Address) -> ZomeApiResult<bool> {
    Ok(hdk::get_entry(&addr)?.is_some())
}
