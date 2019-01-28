use crate::utils::{ Dict, DictKey };

#[derive(PartialEq)]
pub struct State(Dict);

pub type Key = DictKey;

impl Clone for State {
    fn clone(&self) -> Self {
        State(self.0.clone())
    }
}

impl Default for State {
    fn default() -> Self {
        let mut state = Dict::new();
        // any app properties received from coolcats backend
        state.insert("app_properties".into(), Dict::new().into());
        // posts with 'stamp' as their key
        state.insert("posts".into(), Dict::new().into());
        // userHash: handle
        state.insert("handles".into(), Dict::new().into());
        state.insert("handle_taken".into(), false.into());
        // userHash: true
        state.insert("follows".into(), Dict::new().into());
        // active user's handle
        state.insert("handle".into(), "".into());
        // active user's name
        state.insert("first_name".into(), "".into());
        // list of hash posts user has favourited
        state.insert("favourites".into(), Vec::new().into());
        // active user's profile image filename
        state.insert("profile_pic".into(), "".into());
        // active user's userHash
        state.insert("me".into(), "".into());

        State(state)
    }
}

impl State {
    pub fn unset() -> Self {
        State(Dict::new())
    }

    pub fn subset(&self, keys: &[&str]) -> Self {
        State(self.0.subset(keys))
    }

    pub fn get_dict(&self, key: &str) -> Dict {
        self.0.get_dict(key)
    }

    pub fn string(&self, key: &str) -> String {
        self.0.string(key)
    }

    //pub fn strings( &self, key: &str) -> Vec<String>  { self.0.strings(key)  }
    //pub fn integer( &self, key: &str) -> Option<i32>  { self.0.integer(key)  }
    pub fn bool(&self, key: &str) -> Option<bool> {
        self.0.bool(key)
    }

    pub fn mut_dict(&mut self, key: &str) -> &mut Dict {
        self.0.mut_dict(key)
    }

    pub fn set_string(&mut self, key: Key, value: String) {
        self.0.set_string(key, value)
    }

    //pub fn set_strings(&mut self, key: Key, value: Vec<String>) { self.0.set_strings(key, value) }
    //pub fn set_integer(&mut self, key: Key, value: i32)         { self.0.set_integer(key, value) }
    pub fn set_bool(&mut self, key: Key, value: bool) {
        self.0.set_bool(key, value)
    }
}
