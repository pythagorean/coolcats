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

#[macro_export]
macro_rules! class_names {
    ($name:literal) => {
        $name
    };
    ($name:literal => $test:expr) => {
        if $test { $name } else { "" }
    };
    ($testname:expr) => {
        if $testname { stringify!{$testname} } else { "" }
    };
    ($($name:literal),+) => {
        [$(class_names!($name),)+].join(" ")
    };
    ($name1:literal, $name2:literal, $($name3:literal => $test3:expr),+) => {
        [  class_names!($name1),
           class_names!($name2),
         $(class_names!($name3 => $test3),)+].join(" ")
    };
    ($name1:literal, $name2:literal, $($testname3:expr),+) => {
        [  class_names!($name1),
           class_names!($name2),
         $(class_names!($testname3),)+].join(" ")
    };
    ($name1:literal, $($name2:literal => $test2:expr),+) => {
        [  class_names!($name1),
         $(class_names!($name2 => $test2),)+].join(" ")
    };
    ($name1:literal, $name2:literal => $test2:expr, $($testname3:expr),+) => {
        [  class_names!($name1),
           class_names!($name2 => $test2),
         $(class_names!($testname3),)+].join(" ")
    };
    ($name1:literal, $($testname2:expr),+) => {
        [  class_names!($name1),
         $(class_names!($testname2),)+].join(" ")
    };
    ($($name:literal => $test:expr),+) => {
        [$(class_names!($name => $test),)+].join(" ")
    };
    ($name1:literal => $test1:expr, $name2:literal => $test2:expr, $($testname3:expr),+) => {
        [  class_names!($name1 => $test1),
           class_names!($name2 => $test2),
         $(class_names!($testname3),)+].join(" ")
    };
    ($name1:literal => $test1:expr, $($testname2:expr),+) => {
        [  class_names!($name1 => $test1),
         $(class_names!($testname2),)+].join(" ")
    };
    ($($testname:expr),+) => {
        [$(class_names!($testname),)+].join(" ")
    };
}
