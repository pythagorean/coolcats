use serde::{Deserialize, Serialize};

use coolcats_utils::Dict;

#[derive(Serialize, Deserialize, PartialEq)]
enum Status {
    Unset,
    State,
    Substate
}

#[derive(Serialize, Deserialize)]
pub struct State {
    status: Status,
    dict: Dict,
}

impl Default for State {
    fn default() -> Self {
        let mut dict = Dict::new();
        dict.insert("is_uploading".into(), false.into());
        dict.insert("progress".into(), 0.into());
        dict.insert("media_attachments".into(), Vec::new().into());
        Self {
            status: Status::State,
            dict,
        }
    }
}

impl State {
    pub fn unset() -> Self {
        Self {
            status: Status::State,
            dict: Dict::new(),
        }
    }

    pub fn set(&self) -> bool {
        self.status != Status::Unset
    }

    pub fn substate(&self, keys: &[&str]) -> Self {
        if self.status == Status::Unset {
            Self::unset()
        } else {
            Self {
                status: Status::Substate,
                dict: self.dict.subset(keys),
            }
        }
    }

    pub fn strings(&self, key: &str) -> &Vec<String> {
        self.dict.strings(key)
    }

    pub fn integer(&self, key: &str) -> i32 {
        self.dict.integer(key).expect("State::integer called on unset key")
    }

    pub fn bool(&self, key: &str) -> bool {
        self.dict.bool(key).expect("State::bool called on unset key")
    }
}
