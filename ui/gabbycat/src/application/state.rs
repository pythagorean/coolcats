use serde::{Deserialize, Serialize};

use coolcats_utils::Dict;

#[derive(Serialize, Deserialize)]
pub struct State(Dict);

impl Default for State {
    fn default() -> Self {
        let mut dict = Dict::new();
        dict.insert("is_uploading".into(), false.into());
        dict.insert("progress".into(), 0.into());
        dict.insert("media_attachments".into(), Vec::new().into());
        State(dict)
    }
}

impl State {
    pub fn unset() -> Self {
        State(Dict::new())
    }

    pub fn substate(&self, keys: &[&str]) -> Self {
        Self(self.0.subset(keys))
    }

    pub fn strings(&self, key: &str) -> &Vec<String> {
        self.0.strings(key)
    }

    pub fn integer(&self, key: &str) -> i32 {
        self.0.integer(key).expect("State::integer called on unset key")
    }

    pub fn bool(&self, key: &str) -> bool {
        self.0.bool(key).expect("State::bool called on unset key")
    }
}
