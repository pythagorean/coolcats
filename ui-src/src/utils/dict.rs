use std::collections::HashMap;

pub type DictKey = String;

#[derive(PartialEq)]
pub enum DictValue {
    Dict(Dict),
    String(String),
    Bool(bool),
    Vec(Vec<String>),
    Undefined,
}

pub type DictType = HashMap<DictKey, DictValue>;

#[derive(PartialEq)]
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

impl From<bool> for DictValue {
    fn from(value: bool) -> Self {
        DictValue::Bool(value)
    }
}

impl From<Vec<String>> for DictValue {
    fn from(value: Vec<String>) -> Self {
        DictValue::Vec(value)
    }
}

impl Clone for DictValue {
    fn clone(&self) -> Self {
        match self {
            DictValue::Dict(value) => DictValue::Dict((*value).clone()),
            DictValue::String(value) => DictValue::String((*value).clone()),
            DictValue::Bool(value) => DictValue::Bool(*value),
            DictValue::Vec(value) => DictValue::Vec((*value).clone()),
            DictValue::Undefined => DictValue::Undefined,
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

    pub fn insert(&mut self, key: DictKey, value: DictValue) {
        self.0.insert(key, value);
    }

    pub fn dict(&self, key: &str) -> Dict {
        match self.get(key) {
            DictValue::Dict(value) => value,
            DictValue::Undefined => Dict::new(),
            _ => panic! {
                "Dict::dict called on non-dict key"
            }
        }
    }

    //pub fn set_dict(&mut self, key: DictKey, value: Dict) {
    //    self.dict(&key); // force panic if key exists and is not dict
    //    self.insert(key, DictValue::Dict(value));
    //}

    pub fn string(&self, key: &str) -> String {
        match self.get(key) {
            DictValue::String(value) => value,
            DictValue::Undefined => String::new(),
            _ => panic! {
                "Dict::string called on non-string key"
            }
        }
    }

    pub fn set_string(&mut self, key: DictKey, value: String) {
        self.string(&key); // force panic if key exists and is not string
        self.insert(key, DictValue::String(value));
    }

    pub fn bool(&self, key: &str) -> Option<bool> {
        match self.get(key) {
            DictValue::Bool(value) => Some(value),
            DictValue::Undefined => None,
            _ => panic! {
                "Dict::bool called on non-bool key"
            }
        }
    }

    //pub fn set_bool(&mut self, key: DictKey, value: bool) {
    //    self.bool(&key); // force panic if key exists and is not bool
    //    self.insert(key, DictValue::Bool(value));
    //}

    //pub fn vec(&self, key: &str) -> Vec<String> {
    //    match self.get(key) {
    //        DictValue::Vec(value) => value,
    //        DictValue::Undefined => Vec::new(),
    //        _ => panic! {
    //            "Dict::vec called on non-vec key"
    //        }
    //    }
    //}

    //pub fn set_vec(&mut self, key: DictKey, value: Vec<String>) {
    //    self.vec(&key); // force panic if key exists and is not vec
    //    self.insert(key, DictValue::Vec(value));
    //}

    pub fn subset(&self, keys: Vec<&str>) -> Self {
        let mut dict = Dict::new();
        for key in keys {
            if let Some(value) = self.0.get(key) {
                dict.insert((*key).into(), (*value).clone());
            }
        }
        dict
    }

    fn get(&self, key: &str) -> DictValue {
        match self.0.get(key) {
            Some(value) => value.clone(),
            None => DictValue::Undefined,
        }
    }
}
