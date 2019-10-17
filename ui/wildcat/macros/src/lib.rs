mod class_names;
mod locales;
mod state;

pub use wildcat_proc_macros::*;

// Derivable traits and their methods

pub trait UsesLocaleValues {
    fn request_locale_values(&mut self, using_locale_values: Vec<String>);
    fn get_locale_value(&self, message_id: &str) -> &String;
}

pub trait UsesStateValues {
    fn request_state_values(&mut self);
}
