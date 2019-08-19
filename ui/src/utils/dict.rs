use std::collections::HashMap;
use std::collections::HashSet;

use serde::{Serialize, Deserialize};

pub type DictKey = String;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DictValue {
    Dict(Dict),
    String(String),
    Strings(Vec<String>),
    Integer(i32),
    Bool(bool),
}

pub type DictType = HashMap<DictKey, DictValue>;
pub type DictItem = (DictKey, DictValue);
pub type DictList = Vec<DictItem>;

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct Dict(DictType);

impl From<Dict> for DictValue {
    fn from(value: Dict) -> Self {
        DictValue::Dict(value)
    }
}

impl From<String> for DictValue {
    fn from(value: String) -> Self {
        DictValue::String(value)
    }
}

impl From<&str> for DictValue {
    fn from(value: &str) -> Self {
        DictValue::String(value.into())
    }
}

impl From<Vec<String>> for DictValue {
    fn from(value: Vec<String>) -> Self {
        DictValue::Strings(value)
    }
}

impl From<i32> for DictValue {
    fn from(value: i32) -> Self {
        DictValue::Integer(value)
    }
}

impl From<bool> for DictValue {
    fn from(value: bool) -> Self {
        DictValue::Bool(value)
    }
}

impl Clone for DictValue {
    fn clone(&self) -> Self {
        match self {
            DictValue::Dict(value) => DictValue::Dict((*value).clone()),
            DictValue::String(value) => DictValue::String((*value).clone()),
            DictValue::Strings(value) => DictValue::Strings((*value).clone()),
            DictValue::Integer(value) => DictValue::Integer(*value),
            DictValue::Bool(value) => DictValue::Bool(*value),
        }
    }
}

impl Clone for Dict {
    fn clone(&self) -> Self {
        let mut dict = DictType::new();
        for (key, value) in self.0.iter() {
            dict.insert((*key).clone(), (*value).clone());
        }
        Dict(dict)
    }
}

impl Dict {
    pub fn new() -> Self {
        Dict(HashMap::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn raw(&self) -> &DictType {
        &self.0
    }

    pub fn insert(&mut self, key: DictKey, value: DictValue) {
        self.0.insert(key, value);
    }

    pub fn remove(&mut self, key: &str) {
        self.0.remove(key);
    }

    pub fn get_dict(&self, key: &str) -> &Dict {
        lazy_static! {
            static ref EMPTY: Dict = Dict::new();
        }
        match self.get(key) {
            None => &EMPTY,
            Some(DictValue::Dict(value)) => value,
            _ => panic! {
                "Dict::get_dict called on non-dict key"
            },
        }
    }

    pub fn mut_dict(&mut self, key: &str) -> &mut Dict {
        match self.get_mut(key) {
            DictValue::Dict(value) => value,
            _ => panic! {
                "Dict::mut_dict called on non-dict key"
            },
        }
    }

    pub fn string(&self, key: &str) -> &String {
        lazy_static! {
            static ref EMPTY: String = String::new();
        }
        match self.get(key) {
            None => &EMPTY,
            Some(DictValue::String(value)) => value,
            _ => panic! {
                "Dict::string called on non-string key"
            },
        }
    }

    pub fn set_string(&mut self, key: DictKey, value: String) {
        self.string(&key); // force panic if key exists and is not string
        self.insert(key, DictValue::String(value));
    }

    pub fn strings(&self, key: &str) -> &Vec<String> {
        lazy_static! {
            static ref EMPTY: Vec<String> = Vec::new();
        }
        match self.get(key) {
            None => &EMPTY,
            Some(DictValue::Strings(value)) => value,
            _ => panic! {
                "Dict::strings called on non-strings key"
            },
        }
    }

    pub fn set_strings(&mut self, key: DictKey, value: Vec<String>) {
        self.strings(&key); // force panic if key exists and is not strings
        self.insert(key, DictValue::Strings(value));
    }

    //pub fn integer(&self, key: &str) -> Option<i32> {
    //    match self.get(key) {
    //        DictValue::Integer(value) => Some(value),
    //        DictValue::Undefined => None,
    //        _ => panic! {
    //            "Dict::integer called on non-integer key"
    //        }
    //    }
    //}

    //pub fn set_integer(&mut self, key: DictKey, value: integer) {
    //    self.integer(&key); // force panic if key exists and is not integer
    //    self.insert(key, DictValue::Integer(value));
    //}

    pub fn bool(&self, key: &str) -> Option<bool> {
        match self.get(key) {
            None => None,
            Some(DictValue::Bool(value)) => Some(*value),
            _ => panic! {
                "Dict::bool called on non-bool key"
            },
        }
    }

    pub fn set_bool(&mut self, key: DictKey, value: bool) {
        self.bool(&key); // force panic if key exists and is not bool
        self.insert(key, DictValue::Bool(value));
    }

    pub fn subset(&self, keys: &[&str]) -> Self {
        let mut dict = Dict::new();
        let mut uniq = HashSet::new();
        for key in keys {
            if uniq.insert(*key) {
                if let Some(value) = self.get(*key) {
                    dict.insert((*key).into(), (*value).clone());
                }
            }
        }
        dict
    }

    pub fn merge(&mut self, other: &Self) {
        for key in other.0.keys() {
            self.insert(key.clone(), other.get(key).unwrap().clone());
        }
    }

    fn get(&self, key: &str) -> Option<&DictValue> {
        self.0.get(key)
    }

    fn get_mut(&mut self, key: &str) -> &mut DictValue {
        match self.0.get_mut(key) {
            Some(value) => value,
            None => panic! {
                "Dict::get_mut called on nonexistent key"
            },
        }
    }
}
