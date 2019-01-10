use crate::utils::DictValue;

#[derive(PartialEq, Clone)]
pub enum Params {
    Unspecified,
    Positional(Vec<DictValue>),
    Named(Vec<(String, DictValue)>),
}

pub struct WsRpc {
    method: String,
    params: Params,
    id: u32,
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
            Params::Unspecified => {
                r#""params":null"#.to_string()
            },
            Params::Positional(positional_params) => {
                let mut params = Vec::new();
                for param in positional_params {
                    match param {
                        DictValue::String(ref value) => {
                            params.push(format! {
                                r#""{}""#, value
                            });
                        },
                        DictValue::Integer(value) => {
                            params.push(format! {
                                "{}", value
                            });
                        },
                        DictValue::Bool(value) => {
                            params.push(format! {
                                "{}", match value {
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
                    r#""params":[{}]"#,
                    params.join(",")
                }
            },
            Params::Named(named_params) => {
                let mut params = Vec::new();
                for param in named_params {
                    let ref key = param.0;
                    match param.1 {
                        DictValue::String(ref value) => {
                            params.push(format! {
                                r#""{}":"{}""#,
                                key, value
                            });
                        },
                        DictValue::Integer(value) => {
                            params.push(format! {
                                r#""{}":{}"#,
                                key, value
                            });
                        },
                        DictValue::Bool(value) => {
                            params.push(format! {
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

impl From<String> for Call {
    fn from(method: String) -> Self {
        Call {
            method,
            params: Params::Unspecified,
        }
    }
}

impl From<(String, Vec<DictValue>)> for Call {
    fn from(args: (String, Vec<DictValue>)) -> Self {
        Call {
            method: args.0,
            params: Params::Positional(args.1),
        }
    }
}

impl From<(String, Vec<(String, DictValue)>)> for Call {
    fn from(args: (String, Vec<(String, DictValue)>)) -> Self {
        Call {
            method: args.0,
            params: Params::Named(args.1),
        }
    }
}

// No params
impl From<&str> for Call {
    fn from(method: &str) -> Self {
        method.to_string().into()
    }
}

// No params
impl From<&[String]> for Call {
    fn from(method: &[String]) -> Self {
        method.join("/").into()
    }
}

// No params
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

// Named param
impl From<(String, (String, DictValue))> for Call {
    fn from(args: (String, (String, DictValue))) -> Self {
        (args.0, vec![((args.1).0, (args.1).1)]).into()
    }
}

// Positional string param
impl From<(String, String)> for Call {
    fn from(args: (String, String)) -> Self {
        (args.0, DictValue::String(args.1)).into()
    }
}

// Named string param
impl From<(String, (String, String))> for Call {
    fn from(args: (String, (String, String))) -> Self {
        (args.0, ((args.1).0, DictValue::String((args.1).1))).into()
    }
}

// Positional string param
impl From<(&str, &str)> for Call {
    fn from(args: (&str, &str)) -> Self {
        (args.0.to_string(), args.1.to_string()).into()
    }
}

// Named string param
impl From<(&str, (&str, &str))> for Call {
    fn from(args: (&str, (&str, &str))) -> Self {
        (args.0.to_string(), ((args.1).0.to_string(), (args.1).1.to_string())).into()
    }
}

// Positional string param
impl From<(&[&str], &str)> for Call {
    fn from(args: (&[&str], &str)) -> Self {
        (args.0.join("/"), args.1.to_string()).into()
    }
}

// Named string param
impl From<(&[&str], (&str, &str))> for Call {
    fn from(args: (&[&str], (&str, &str))) -> Self {
        (args.0.join("/"), ((args.1).0.to_string(), (args.1).1.to_string())).into()
    }
}

// Positional param
impl From<(&str, DictValue)> for Call {
    fn from(args: (&str, DictValue)) -> Self {
        (args.0.to_string(), args.1).into()
    }
}

// Named param
impl From<(&str, (&str, DictValue))> for Call {
    fn from(args: (&str, (&str, DictValue))) -> Self {
        (args.0.to_string(), ((args.1).0.to_string(), (args.1).1)).into()
    }
}

// Positional integer param
impl From<(&str, i32)> for Call {
    fn from(args: (&str, i32)) -> Self {
        (args.0.to_string(), DictValue::Integer(args.1)).into()
    }
}

// Named integer param
impl From<(&str, (&str, i32))> for Call {
    fn from(args: (&str, (&str, i32))) -> Self {
        (args.0.to_string(), ((args.1).0.to_string(), DictValue::Integer((args.1).1))).into()
    }
}

// Positional integer param
impl From<(&[&str], i32)> for Call {
    fn from(args: (&[&str], i32)) -> Self {
        (args.0.join("/"), DictValue::Integer(args.1)).into()
    }
}

// Named integer param
impl From<(&[&str], (&str, i32))> for Call {
    fn from(args: (&[&str], (&str, i32))) -> Self {
        (args.0.join("/"), ((args.1).0.to_string(), DictValue::Integer((args.1).1))).into()
    }
}

// Positional bool param
impl From<(&str, bool)> for Call {
    fn from(args: (&str, bool)) -> Self {
        (args.0.to_string(), DictValue::Bool(args.1)).into()
    }
}

// Named bool param
impl From<(&str, (&str, bool))> for Call {
    fn from(args: (&str, (&str, bool))) -> Self {
        (args.0.to_string(), ((args.1).0.to_string(), DictValue::Bool((args.1).1))).into()
    }
}

// Positional bool param
impl From<(&[&str], bool)> for Call {
    fn from(args: (&[&str], bool)) -> Self {
        (args.0.join("/"), DictValue::Bool(args.1)).into()
    }
}

// Named bool param
impl From<(&[&str], (&str, bool))> for Call {
    fn from(args: (&[&str], (&str, bool))) -> Self {
        (args.0.join("/"), ((args.1).0.to_string(), DictValue::Bool((args.1).1))).into()
    }
}

// Positional string params
impl From<(String, &[String])> for Call {
    fn from(args: (String, &[String])) -> Self {
        let method = args.0;
        let params: Vec<_> = args.1.iter()
            .map(|value| DictValue::String(value.to_string()))
            .collect();
        (method, params).into()
    }
}

// Positional integer params
impl From<(String, &[i32])> for Call {
    fn from(args: (String, &[i32])) -> Self {
        let method = args.0;
        let params: Vec<_> = args.1.iter()
            .map(|value| DictValue::Integer(*value))
            .collect();
        (method, params).into()
    }
}

// Named string params
impl From<(String, &[(String, String)])> for Call {
    fn from(args: (String, &[(String, String)])) -> Self {
        let method = args.0;
        let params: Vec<_> = args.1.iter()
            .map(|(key, value)| (key.clone(), DictValue::String(value.to_string())))
            .collect();
        (method, params).into()
    }
}

// Positional string params
impl From<(String, &[&str])> for Call {
    fn from(args: (String, &[&str])) -> Self {
        let method = args.0;
        let params: Vec<_> = args.1.iter()
            .map(|value| DictValue::String(value.to_string()))
            .collect();
        (method, params).into()
    }
}

// Named string params
impl From<(String, &[(&str, &str)])> for Call {
    fn from(args: (String, &[(&str, &str)])) -> Self {
        let method = args.0;
        let params: Vec<_> = args.1.iter()
            .map(|(key, value)| (key.to_string(), DictValue::String(value.to_string())))
            .collect();
        (method, params).into()
    }
}

// Positional params
impl From<(String, &[DictValue])> for Call {
    fn from(args: (String, &[DictValue])) -> Self {
        let method = args.0;
        let params: Vec<_> = args.1.iter()
            .map(|value| value.clone())
            .collect();
        (method, params).into()
    }
}

// Named params
impl From<(String, &[(&str, DictValue)])> for Call {
    fn from(args: (String, &[(&str, DictValue)])) -> Self {
        let method = args.0;
        let params: Vec<_> = args.1.iter()
            .map(|(key, value)| (key.to_string(), value.clone()))
            .collect();
        (method, params).into()
    }
}

// Positional params
impl From<(&str, &[DictValue])> for Call {
    fn from(args: (&str, &[DictValue])) -> Self {
        (args.0.to_string(), args.1).into()
    }
}

// Named params
impl From<(&str, &[(String, DictValue)])> for Call {
    fn from(args: (&str, &[(String, DictValue)])) -> Self {
        (args.0.into(), args.1).into()
    }
}

// Named params
impl From<(&str, &[(&str, DictValue)])> for Call {
    fn from(args: (&str, &[(&str, DictValue)])) -> Self {
        (args.0.to_string(), args.1).into()
    }
}

// Positional string params
impl From<(&str, &[&str])> for Call {
    fn from(args: (&str, &[&str])) -> Self {
        (args.0.to_string(), args.1).into()
    }
}

// Positional integer params
impl From<(&str, &[i32])> for Call {
    fn from(args: (&str, &[i32])) -> Self {
        (args.0.to_string(), args.1).into()
    }
}

// Named string params
impl From<(&str, &[(&str, &str)])> for Call {
    fn from(args: (&str, &[(&str, &str)])) -> Self {
        (args.0.to_string(), args.1).into()
    }
}

// Positional string params
impl From<(&[&str], &[&str])> for Call {
    fn from(args: (&[&str], &[&str])) -> Self {
        (args.0.join("/"), args.1).into()
    }
}

// Positional integer params
impl From<(&[&str], &[i32])> for Call {
    fn from(args: (&[&str], &[i32])) -> Self {
        (args.0.join("/"), args.1).into()
    }
}

// Named string params
impl From<(&[&str], &[(&str, &str)])> for Call {
    fn from(args: (&[&str], &[(&str, &str)])) -> Self {
        (args.0.join("/"), args.1).into()
    }
}

// Positional params
impl From<(&[&str], &[DictValue])> for Call {
    fn from(args: (&[&str], &[DictValue])) -> Self {
        (args.0.join("/"), args.1).into()
    }
}

// Named params
impl From<(&[&str], &[(&str, DictValue)])> for Call {
    fn from(args: (&[&str], &[(&str, DictValue)])) -> Self {
        (args.0.join("/"), args.1).into()
    }
}
