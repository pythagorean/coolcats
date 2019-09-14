// For now variadic functions use macro_rules and boilerplate uses derive

pub use gabbycat_macros_derive::*;

pub trait UsesLocaleValues {
    fn request_locale_values(&mut self);
    fn get_locale_value(&self, message_id: &str) -> &String;
}

pub trait LocaleComponent {}

#[macro_export]
macro_rules! use_locale_values {
    ($($x: expr),+) => {
        pub fn using_locale_values() -> Vec<String> {
            use lazy_static::*;
            lazy_static! {
                static ref VS: Vec<String> = {
                    let mut vector = Vec::new();
                    $(vector.push($x.to_string());)+
                    vector
                };
            }
            VS.to_vec()
        }
    };
}

pub trait UsesStateValues {
    fn request_state_values(&mut self);
}

#[macro_export]
macro_rules! use_state_values {
    ($($x: expr),+) => {
        pub fn using_state_values() -> Vec<String> {
            use lazy_static::*;
            lazy_static! {
                static ref VS: Vec<String> = {
                    let mut vector = Vec::new();
                    $(vector.push($x.to_string());)+
                    vector
                };
            }
            VS.to_vec()
        }
    };
}
