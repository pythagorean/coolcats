use serde::{Deserialize, Serialize};

use coolcats_utils::Dict;

#[derive(Serialize, Deserialize)]
pub struct State(Dict);

impl Default for State {
    fn default() -> Self {
        let mut dict = Dict::new();
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
}
