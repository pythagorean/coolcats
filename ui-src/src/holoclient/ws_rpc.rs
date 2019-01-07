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
                r#""params":null"#.into()
            },
            Params::Positional(positional_params) => {
                let mut params = Vec::new();
                for param in positional_params {
                    match param {
                        DictValue::String(ref value) => {
                            params.insert(0, format! {
                                r#""{}""#, value
                            });
                        },
                        DictValue::Integer(value) => {
                            params.insert(0, format! {
                                "{}", value
                            });
                        },
                        DictValue::Bool(value) => {
                            params.insert(0, format! {
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
        method.into()
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
        (
            args.0,
            {
                let mut params: Vec<String> = Vec::new();
                for param in args.1 {
                    params.insert(0, param.clone().into());
                }
                params
            }.as_slice()
        ).into()
    }
}

// Named string params
impl From<(String, &[(String, String)])> for Call {
    fn from(args: (String, &[(String, String)])) -> Self {
        (
            args.0,
            {
                let mut params = Vec::new();
                for param in args.1 {
                    params.insert(0, (param.0.clone(), param.1.clone().into()));
                }
                params
            }.as_slice()
        ).into()
    }
}

// Positional string params
impl From<(String, &[&str])> for Call {
    fn from(args: (String, &[&str])) -> Self {
        (
            args.0,
            {
                let mut params: Vec<String> = Vec::new();
                for param in args.1 {
                    params.insert(0, param.to_string());
                }
                params
            }.as_slice()
        ).into()
    }
}

// Named string params
impl From<(String, &[(&str, &str)])> for Call {
    fn from(args: (String, &[(&str, &str)])) -> Self {
        (
            args.0,
            {
                let mut params = Vec::new();
                for param in args.1 {
                    params.insert(0, (param.0.to_string(), param.1.into()));
                }
                params
            }.as_slice()
        ).into()
    }
}

// Positional params
impl From<(String, &[DictValue])> for Call {
    fn from(args: (String, &[DictValue])) -> Self {
        (
            args.0,
            {
                let mut params = Vec::new();
                for param in args.1 {
                    params.insert(0, param.clone());
                }
                params
            }.as_slice()
        ).into()
    }
}

// Named params
impl From<(String, &[(&str, DictValue)])> for Call {
    fn from(args: (String, &[(&str, DictValue)])) -> Self {
        (
            args.0,
            {
                let mut params = Vec::new();
                for param in args.1 {
                    params.insert(0, (param.0.into(), param.1.clone()));
                }
                params
            }.as_slice()
        ).into()
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
