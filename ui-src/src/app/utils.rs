use std::collections::HashMap;

pub type DictKey = String;

pub enum DictValue {
    Dict(Dict),
    String(String),
    Bool(bool),
    Vec(Vec<String>),
}

pub type DictType = HashMap<DictKey, DictValue>;

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
        if let DictValue::Dict(value) = self.get(key) {
            return (*value).clone();
        } else {
            panic! { "Dict::dict called on non-dict key" };
        }
    }

    pub fn string(&self, key: &str) -> String {
        if let DictValue::String(value) = self.get(key) {
            return (*value).clone();
        } else {
            panic! { "Dict::string called on non-string key" };
        }
    }

    pub fn bool(&self, key: &str) -> bool {
        if let DictValue::Bool(value) = self.get(key) {
            return *value;
        } else {
            panic! { "Dict::bool called on non-bool key" };
        }
    }

    pub fn vec(&self, key: &str) -> Vec<String> {
        if let DictValue::Vec(value) = self.get(key) {
            return (*value).clone();
        } else {
            panic! { "Dict::vec called on non-vec key" };
        }
    }

    fn get(&self, key: &str) -> &DictValue {
        self.0.get(key).unwrap()
    }
}
