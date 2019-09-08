use fluent::{FluentBundle, FluentResource};
use unic_langid::langid;

pub struct Locale;

impl Locale {
    pub fn initialize() -> super::Locale {
        let langid_en = langid!("en-US");
        let mut bundle = FluentBundle::new(&[langid_en]);
        bundle
            .add_resource(
                FluentResource::try_new(String::from(include_str!("en.ftl")))
                    .expect("Failed to parse an FTL string."),
            )
            .expect("Failed to add FTL resources to the bundle.");
        super::Locale { bundle }
    }
}
