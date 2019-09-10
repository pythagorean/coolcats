use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct State(HashMap<String, String>);

impl State {
    pub fn initialize() -> Self {
        Self(HashMap::new())
    }

    pub fn substate(&self, _keys: &[&str]) -> Self {
        Self(HashMap::new())
    }
}
