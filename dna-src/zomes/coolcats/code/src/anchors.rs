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

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Anchor {
    anchor_type: String,
    anchor_text: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct AnchorLink {
    base: HashString,
    link: HashString,
    tag: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct AnchorLinks {
    links: Vec<AnchorLink>,
}

pub fn anchor_definition() -> ValidatingEntryType {
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

pub fn anchor_links_definition() -> ValidatingEntryType {
    entry!(
        name: "anchor_links",
        description: "A list of anchor link types",
        sharing: Sharing::Public,
        native_type: AnchorLinks,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_anchor_links: AnchorLinks, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

pub fn anchor(anchor_type: &str, anchor_text: &str) -> ZomeApiResult<HashString> {
    let anchor_entry = Entry::new(EntryType::App("anchor".into()), Anchor {
        anchor_type: anchor_type.into(),
        anchor_text: anchor_text.into()
    });
    let anchor_address = hdk::entry_address(&anchor_entry)?;
    if hdk::get_entry(anchor_address.clone())?.is_some() {
        return Ok(anchor_address);
    }

    let anchor_type_entry = Entry::new(EntryType::App("anchor".into()), Anchor {
        anchor_type: anchor_type.into(),
        anchor_text: "".into()
    });
    let anchor_type_address = hdk::entry_address(&anchor_type_entry)?;
    if hdk::get_entry(anchor_type_address.clone())?.is_none() {
        let root_anchor_type_entry = Entry::new(EntryType::App("anchor".into()), Anchor {
            anchor_type: "anchor_types".into(),
            anchor_text: "".into()
        });
        let root_anchor_type_address = hdk::entry_address(&root_anchor_type_entry)?;
        if hdk::get_entry(root_anchor_type_address.clone())?.is_none() {
            hdk::commit_entry(&root_anchor_type_entry)?;
        }

        let anchor_link_entry = Entry::new(EntryType::App("anchor_links".into()), AnchorLinks {
            links: vec![ AnchorLink {
                base: root_anchor_type_address.clone(),
                link: anchor_type_address.clone(),
                tag: anchor_type.into()
            }]
        });
        hdk::commit_entry(&anchor_link_entry)?;
        hdk::commit_entry(&anchor_type_entry)?;
    }
    let anchor_link_entry = Entry::new(EntryType::App("anchor_links".into()), AnchorLinks {
        links: vec![ AnchorLink {
            base: anchor_type_address.clone(),
            link: anchor_address.clone(),
            tag: anchor_text.into()
        }]
    });
    hdk::commit_entry(&anchor_link_entry)?;
    hdk::commit_entry(&anchor_entry)
}

pub fn anchor_exists(anchor_type: &str, anchor_text: &str) -> ZomeApiResult<bool> {
    let entry = Entry::new(EntryType::App("anchor".into()), Anchor {
        anchor_type: anchor_type.into(),
        anchor_text: anchor_text.into(),
    });
    Ok(hdk::get_entry(hdk::entry_address(&entry)?)?.is_some())
}
