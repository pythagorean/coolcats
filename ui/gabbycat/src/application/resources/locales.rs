pub mod en;

use fluent::{FluentBundle, FluentResource};
use std::borrow::Cow;

pub struct Locale {
    bundle: FluentBundle<FluentResource>,
}

impl Locale {
    pub fn get_value(&self, message_id: &str) -> Cow<str> {
        let message = self.bundle.get_message(message_id).expect("Message doesn't exist.");
        let pattern = message.value.expect("Message has no value.");
        let mut errors = vec![];
        self.bundle.format_pattern(&pattern, None, &mut errors)
    }
}
