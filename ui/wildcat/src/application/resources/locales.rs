pub mod en;

use fluent::{FluentBundle, FluentResource};
use std::borrow::Cow;
use stdweb::js;

pub struct Locale {
    bundle: FluentBundle<FluentResource>,
}

impl Locale {
    pub fn get_value(&self, message_id: &str) -> Cow<str> {
        let mut return_value = Cow::Borrowed("");
        if let Some(message) = self.bundle.get_message(message_id) {
            if let Some(pattern) = message.value {
                let mut errors = vec![];
                return_value = self.bundle.format_pattern(&pattern, None, &mut errors);
            } else {
                js! { console.debug(@{format!("Message ({}) has no value.", message_id)}) };
            }
        } else {
            js! { console.debug(@{format!("Message ({}) doesn't exist.", message_id)}) };
        }
        return_value
    }
}
