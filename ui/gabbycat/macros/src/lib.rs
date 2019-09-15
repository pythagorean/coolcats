// For now variadic functions use macro_rules and boilerplate uses derive

mod class_names;
mod locales;
mod state;

pub use gabbycat_macros_derive::*;

pub trait UsesLocaleValues {
    fn request_locale_values(&mut self);
    fn get_locale_value(&self, message_id: &str) -> &String;
}

pub trait LocaleComponent {}

pub trait UsesStateValues {
    fn request_state_values(&mut self);
}
