trait UsesLocaleValues {
    fn request_locale_values(&mut self);
    fn get_locale_value(&self, message_id: &str) -> &String;
}

macro_rules! use_locale_values {
    ($($x: expr),+) => {
        pub fn using_locale_values() -> Vec<String> {
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

pub mod home;
