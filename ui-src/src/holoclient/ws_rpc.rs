use crate::utils::DictValue;

pub struct WsRpc {
    method: String,
    params: Vec<(String, DictValue)>, // Only handles DictValue::String for now
    id: u32,
}

impl WsRpc {
    pub fn json(&self) -> String {
        let method = format! {
            r#""method":"{}""#,
            self.method
        };
        let params = match self.params.is_empty() {
            true => r#""params":null"#.into(),
            false => {
                let mut params = Vec::new();
                for param in &self.params {
                    if let DictValue::String(ref value) = param.1 {
                        let ref key = param.0;
                        params.insert(0, format! {
                            r#""{}":"{}""#,
                            key, value
                        });
                    }
                }
                format! {
                    r#""params":{{{}}}"#,
                    params.join(",")
                }
            },
        };
        let id = format! {
            r#""id":{}"#,
            self.id
        };
        format! {
            r#"{{"jsonrpc":"2.0",{},{},{}}}"#,
            method, params, id
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Call {
    method: String,
    params: Vec<(String, DictValue)>,
}

impl Call {
    pub fn new() -> Self {
        Call {
            method: String::new(),
            params: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.method.clear();
        self.params.clear();
    }

    pub fn has_method(&self) -> bool {
        !self.method.is_empty()
    }
}

impl From<&str> for Call {
    fn from(method: &str) -> Self {
        Call {
            method: method.into(),
            params: Vec::new(),
        }
    }
}

impl From<Vec<&str>> for Call {
    fn from(method: Vec<&str>) -> Self {
        Call {
            method: method.join("/"),
            params: Vec::new(),
        }
    }
}

impl From<(&str, (&str, &str))> for Call {
    fn from(args: (&str, (&str, &str))) -> Self {
        Call {
            method: args.0.into(),
            params: vec![((args.1).0.into(), (args.1).1.into())],
        }
    }
}

impl From<(Vec<&str>, (&str, &str))> for Call {
    fn from(args: (Vec<&str>, (&str, &str))) -> Self {
        Call {
            method: args.0.join("/"),
            params: vec![((args.1).0.into(), (args.1).1.into())],
        }
    }
}

impl From<(Vec<&str>, Vec<(&str, &str)>)> for Call {
    fn from(args: (Vec<&str>, Vec<(&str, &str)>)) -> Self {
        Call {
            method: args.0.join("/"),
            params: match args.1.is_empty() {
                true => Vec::new(),
                false => {
                    let mut params = Vec::new();
                    for param in args.1 {
                        params.insert(0, (param.0.into(), param.1.into()));
                    }
                    params
                }
            },
        }
    }
}

impl From<(Call, u32)> for WsRpc {
    fn from(call_id: (Call, u32)) -> Self {
        WsRpc {
            method: call_id.0.method,
            params: call_id.0.params,
            id: call_id.1,
        }
    }
}
