use crate::utils::{DictValue, DictItem, DictList};

#[derive(PartialEq, Clone)]
pub enum Params {
    Unspecified,
    Positional(Vec<DictValue>),
    Named(DictList),
}

pub struct WsRpc {
    method: String,
    params: Params,
    id: String,
}

#[derive(PartialEq, Clone)]
pub struct Call {
    method: String,
    params: Params,
}

impl From<(Call, u32)> for WsRpc {
    fn from(call_id: (Call, u32)) -> Self {
        WsRpc {
            method: call_id.0.method,
            params: call_id.0.params,
            id: call_id.1.to_string(),
        }
    }
}

impl From<(Call, String)> for WsRpc {
    fn from(call_id: (Call, String)) -> Self {
        WsRpc {
            method: call_id.0.method,
            params: call_id.0.params,
            id: call_id.1,
        }
    }
}

impl WsRpc {
    pub fn json(&self) -> String {
        let method = format! {
            r#""method":"{}""#,
            self.method
        };
        let params = match &self.params {
            Params::Unspecified => r#""params":null"#.to_string(),
            Params::Positional(positional_params) => {
                let mut params = Vec::new();
                for param in positional_params {
                    match param {
                        DictValue::String(ref value) => {
                            params.push(format! {
                                r#""{}""#, value
                            });
                        }
                        DictValue::Strings(ref value) => {
                            params.push(format! {
                                r#"["{}"]"#, value.join(r#"",""#)
                            });
                        }
                        DictValue::Integer(value) => {
                            params.push(value.to_string());
                        }
                        DictValue::Bool(value) => {
                            params.push(
                                if *value {
                                    "true"
                                } else {
                                    "false"
                                }
                                .to_string(),
                            );
                        }
                        _ => {
                            panic! { "Unsupported RPC parameter type" };
                        }
                    }
                }
                format! {
                    r#""params":[{}]"#,
                    params.join(",")
                }
            }
            Params::Named(named_params) => {
                let mut params = Vec::new();
                for param in named_params {
                    let key = &param.0;
                    match param.1 {
                        DictValue::String(ref value) => {
                            params.push(format! {
                                r#""{}":"{}""#,
                                key, value
                            });
                        }
                        DictValue::Strings(ref value) => {
                            params.push(format! {
                                r#""{}":["{}"]"#,
                                key, value.join(r#"",""#)
                            });
                        }
                        DictValue::Integer(value) => {
                            params.push(format! {
                                r#""{}":{}"#,
                                key, value
                            });
                        }
                        DictValue::Bool(value) => {
                            params.push(format! {
                                r#""{}":{}"#,
                                key, if value { "true" } else { "false" }
                            });
                        }
                        _ => {
                            panic! { "Unsupported RPC parameter type" };
                        }
                    }
                }
                format! {
                    r#""params":{{{}}}"#,
                    params.join(",")
                }
            }
        };
        let id = format! {
            r#""id":"{}""#,
            self.id
        };
        format! {
            r#"{{"jsonrpc":"2.0",{},{},{}}}"#,
            method, params, id
        }
    }
}

impl Call {
    pub fn new() -> Self {
        Call {
            method: String::new(),
            params: Params::Unspecified,
        }
    }

    pub fn clear(&mut self) {
        self.method.clear();
        self.params = Params::Unspecified;
    }

    pub fn has_method(&self) -> bool {
        !self.method.is_empty()
    }
}

// No params

impl From<String> for Call {
    fn from(method: String) -> Self {
        Call {
            method,
            params: Params::Unspecified,
        }
    }
}

impl From<&str> for Call {
    fn from(method: &str) -> Self {
        method.to_string().into()
    }
}

impl From<&[&str]> for Call {
    fn from(method: &[&str]) -> Self {
        method.join("/").into()
    }
}

// Positional param

impl From<(String, DictValue)> for Call {
    fn from(args: (String, DictValue)) -> Self {
        (args.0, vec![args.1]).into()
    }
}

impl From<(&str, DictValue)> for Call {
    fn from(args: (&str, DictValue)) -> Self {
        (args.0.to_string(), args.1).into()
    }
}

impl From<(&str, &DictValue)> for Call {
    fn from(args: (&str, &DictValue)) -> Self {
        (args.0, args.1.clone()).into()
    }
}

impl From<(&[&str], DictValue)> for Call {
    fn from(args: (&[&str], DictValue)) -> Self {
        (args.0.join("/"), args.1).into()
    }
}

impl From<(&[&str], &DictValue)> for Call {
    fn from(args: (&[&str], &DictValue)) -> Self {
        (args.0, args.1.clone()).into()
    }
}

// Positional params

impl From<(String, Vec<DictValue>)> for Call {
    fn from(args: (String, Vec<DictValue>)) -> Self {
        Call {
            method: args.0,
            params: Params::Positional(args.1),
        }
    }
}

impl From<(&str, Vec<DictValue>)> for Call {
    fn from(args: (&str, Vec<DictValue>)) -> Self {
        (args.0.to_string(), args.1).into()
    }
}

impl From<(&str, &[DictValue])> for Call {
    fn from(args: (&str, &[DictValue])) -> Self {
        (args.0, args.1.to_vec()).into()
    }
}

impl From<(&[&str], Vec<DictValue>)> for Call {
    fn from(args: (&[&str], Vec<DictValue>)) -> Self {
        (args.0.join("/"), args.1).into()
    }
}

impl From<(&[&str], &[DictValue])> for Call {
    fn from(args: (&[&str], &[DictValue])) -> Self {
        (args.0, args.1.to_vec()).into()
    }
}

// Named param

impl From<(String, DictItem)> for Call {
    fn from(args: (String, DictItem)) -> Self {
        (args.0, vec![args.1]).into()
    }
}

impl From<(&str, DictItem)> for Call {
    fn from(args: (&str, DictItem)) -> Self {
        (args.0.to_string(), args.1).into()
    }
}

impl From<(&str, &DictItem)> for Call {
    fn from(args: (&str, &DictItem)) -> Self {
        (args.0, args.1.clone()).into()
    }
}

impl From<(&[&str], DictItem)> for Call {
    fn from(args: (&[&str], DictItem)) -> Self {
        (args.0.join("/"), args.1).into()
    }
}

impl From<(&[&str], &DictItem)> for Call {
    fn from(args: (&[&str], &DictItem)) -> Self {
        (args.0, args.1.clone()).into()
    }
}

// Named params

impl From<(String, DictList)> for Call {
    fn from(args: (String, DictList)) -> Self {
        Call {
            method: args.0,
            params: Params::Named(args.1),
        }
    }
}

impl From<(&str, DictList)> for Call {
    fn from(args: (&str, DictList)) -> Self {
        (args.0.to_string(), args.1).into()
    }
}

impl From<(&str, &[DictItem])> for Call {
    fn from(args: (&str, &[DictItem])) -> Self {
        (args.0, args.1.to_vec()).into()
    }
}

impl From<(&[&str], DictList)> for Call {
    fn from(args: (&[&str], DictList)) -> Self {
        (args.0.join("/"), args.1).into()
    }
}

impl From<(&[&str], &[DictItem])> for Call {
    fn from(args: (&[&str], &[DictItem])) -> Self {
        (args.0, args.1.to_vec()).into()
    }
}
