use serde::{Deserialize, Serialize};

use coolcats_utils::Dict;

#[derive(Serialize, Deserialize)]
pub struct State(Dict);

impl State {
    pub fn initialize() -> Self {
        Self(Dict::new())
    }

    pub fn substate(&self, keys: &[&str]) -> Self {
        Self(self.0.subset(keys))
    }
}
