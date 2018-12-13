use hdk::{
    self,
    entry_definition::ValidatingEntryType,
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
pub struct Link {
    base: HashString,
    link: HashString,
    tag: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Links {
    name: String,
    links: Vec<Link>,
}

impl Link {
    pub fn new(base: &HashString, link: &HashString, tag: &str) -> Link {
        Link {
            base: base.to_owned(),
            link: link.to_owned(),
            tag: tag.to_owned(),
        }
    }
}

impl Links {
    pub fn definition() -> ValidatingEntryType {
        entry!(
            name: "links",
            description: "A list of link types",
            sharing: Sharing::Public,
            native_type: Links,

            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: |_links: Links, _ctx: hdk::ValidationData| {
                Ok(())
            }
        )
    }

    pub fn new(name: &str, link: Link) -> Links {
        Links {
            name: name.to_owned(),
            links: vec![link],
        }
    }

    pub fn entry(name: &str, link: Link) -> Entry {
        Entry::new(
            EntryType::App("links".into()),
            Links::new(name, link)
        )
    }
}
