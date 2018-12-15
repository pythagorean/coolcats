use hdk::{
    self,
    error::ZomeApiResult,
    holochain_core_types::cas::content::Address,
};

pub fn address_exists(address: &Address) -> ZomeApiResult<bool> {
    Ok(hdk::get_entry((*address).clone())?.is_some())
}

//use hdk::entry::Entry;
//pub fn entry_exists(entry: &Entry) -> ZomeApiResult<bool> {
//    address_exists(&hdk::entry_address(entry)?)
//}
