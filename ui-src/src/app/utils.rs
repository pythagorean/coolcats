use std::collections::HashMap;

pub type DictType = HashMap<DictKey, DictValue>;

pub type DictKey = String;

pub enum DictValue {
    Dict(Dict),
    String(String),
    Bool(bool),
}

pub struct Dict {
    dict: DictType,
}

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

impl Clone for DictValue {
    fn clone(&self) -> DictValue {
        match self {
            DictValue::Dict(value) => DictValue::Dict((*value).clone()),
            DictValue::String(value) => DictValue::String((*value).clone()),
            DictValue::Bool(value) => DictValue::Bool(*value),
        }
    }
}

impl Clone for Dict {
    fn clone(&self) -> Dict {
        let mut dict = DictType::new();
        for (key, value) in self.dict.iter() {
            dict.insert((*key).clone(), (*value).clone());
        }
        Dict { dict: dict }
    }
}

impl Dict {
    pub fn new() -> Self {
        Dict {
            dict: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.dict.clear();
    }

    pub fn insert(&mut self, key: DictKey, value: DictValue) {
        self.dict.insert(key, value);
    }

    pub fn dict(&self, key: DictKey) -> Dict {
        if let DictValue::Dict(value) = self.get(key) {
            return (*value).clone();
        }
        Dict::new()
    }

    pub fn string(&self, key: DictKey) -> String {
        if let DictValue::String(value) = self.get(key) {
            return (*value).clone();
        }
        "".into()
    }

    pub fn bool(&self, key: DictKey) -> bool {
        if let DictValue::Bool(value) = self.get(key) {
            return *value;
        }
        false
    }

    fn get(&self, key: DictKey) -> &DictValue {
        self.dict.get(&key).unwrap()
    }
}
