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
