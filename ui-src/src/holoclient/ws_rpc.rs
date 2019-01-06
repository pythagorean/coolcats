use crate::utils::DictValue;

pub struct WsRpc {
    method: String,
    params: Vec<(String, DictValue)>,
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
                    let ref key = param.0;
                    match param.1 {
                        DictValue::String(ref value) => {
                            params.insert(0, format! {
                                r#""{}":"{}""#,
                                key, value
                            });
                        },
                        DictValue::Integer(value) => {
                            params.insert(0, format! {
                                r#""{}":{}"#,
                                key, value
                            });
                        },
                        DictValue::Bool(value) => {
                            params.insert(0, format! {
                                r#""{}":{}"#,
                                key, match value {
                                    true => "true",
                                    false => "false",
                                }
                            });
                        },
                        _ => {
                            panic! { "Unsupported RPC parameter type" };
                        },
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

impl From<String> for Call {
    fn from(method: String) -> Self {
        Call {
            method: method,
            params: Vec::new(),
        }
    }
}

impl From<&str> for Call {
    fn from(method: &str) -> Self {
        method.into()
    }
}

impl From<Vec<String>> for Call {
    fn from(method: Vec<String>) -> Self {
        method.join("/").into()
    }
}

impl From<Vec<&str>> for Call {
    fn from(method: Vec<&str>) -> Self {
        method.join("/").into()
    }
}

impl From<(String, (String, DictValue))> for Call {
    fn from(args: (String, (String, DictValue))) -> Self {
        Call {
            method: args.0,
            params: vec![((args.1).0, (args.1).1)]
        }
    }
}

impl From<(String, (String, String))> for Call {
    fn from(args: (String, (String, String))) -> Self {
        (args.0, ((args.1).0, DictValue::String((args.1).1))).into()
    }
}

impl From<(&str, (&str, &str))> for Call {
    fn from(args: (&str, (&str, &str))) -> Self {
        (args.0.to_string(), ((args.1).0.to_string(), (args.1).1.to_string())).into()
    }
}

impl From<(Vec<&str>, (&str, &str))> for Call {
    fn from(args: (Vec<&str>, (&str, &str))) -> Self {
        (args.0.join("/"), ((args.1).0.to_string(), (args.1).1.to_string())).into()
    }
}

impl From<(&str, (&str, DictValue))> for Call {
    fn from(args: (&str, (&str, DictValue))) -> Self {
        (args.0.to_string(), ((args.1).0.to_string(), (args.1).1)).into()
    }
}

impl From<(&str, (&str, i32))> for Call {
    fn from(args: (&str, (&str, i32))) -> Self {
        (args.0.to_string(), ((args.1).0.to_string(), DictValue::Integer((args.1).1))).into()
    }
}

impl From<(Vec<&str>, (&str, i32))> for Call {
    fn from(args: (Vec<&str>, (&str, i32))) -> Self {
        (args.0.join("/"), ((args.1).0.to_string(), DictValue::Integer((args.1).1))).into()
    }
}

impl From<(&str, (&str, bool))> for Call {
    fn from(args: (&str, (&str, bool))) -> Self {
        (args.0.to_string(), ((args.1).0.to_string(), DictValue::Bool((args.1).1))).into()
    }
}

impl From<(Vec<&str>, (&str, bool))> for Call {
    fn from(args: (Vec<&str>, (&str, bool))) -> Self {
        (args.0.join("/"), ((args.1).0.to_string(), DictValue::Bool((args.1).1))).into()
    }
}

impl From<(String, Vec<(String, String)>)> for Call {
    fn from(args: (String, Vec<(String, String)>)) -> Self {
        (args.0, {
            let mut params = Vec::new();
            for param in args.1 {
                params.insert(0, (param.0, param.1.into()));
            }
            params
        }).into()
    }
}

impl From<(String, Vec<(&str, &str)>)> for Call {
    fn from(args: (String, Vec<(&str, &str)>)) -> Self {
        (args.0, {
            let mut params = Vec::new();
            for param in args.1 {
                params.insert(0, (param.0.to_string(), param.1.into()));
            }
            params
        }).into()
    }
}

impl From<(String, Vec<(&str, DictValue)>)> for Call {
    fn from(args: (String, Vec<(&str, DictValue)>)) -> Self {
        (args.0, {
            let mut params = Vec::new();
            for param in args.1 {
                params.insert(0, (param.0.into(), param.1));
            }
            params
        }).into()
    }
}

impl From<(&str, Vec<(String, DictValue)>)> for Call {
    fn from(args: (&str, Vec<(String, DictValue)>)) -> Self {
        (args.0.into(), args.1).into()
    }
}

impl From<(&str, Vec<(&str, DictValue)>)> for Call {
    fn from(args: (&str, Vec<(&str, DictValue)>)) -> Self {
        (args.0.to_string(), args.1).into()
    }
}

impl From<(&str, Vec<(&str, &str)>)> for Call {
    fn from(args: (&str, Vec<(&str, &str)>)) -> Self {
        (args.0.to_string(), args.1).into()
    }
}

impl From<(Vec<&str>, Vec<(&str, &str)>)> for Call {
    fn from(args: (Vec<&str>, Vec<(&str, &str)>)) -> Self {
        (args.0.join("/"), args.1).into()
    }
}

impl From<(Vec<&str>, Vec<(&str, DictValue)>)> for Call {
    fn from(args: (Vec<&str>, Vec<(&str, DictValue)>)) -> Self {
        (args.0.join("/"), args.1).into()
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
