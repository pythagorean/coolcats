use crate::utils::{DictValue, DictItem, DictList};

#[derive(PartialEq, Clone)]
pub enum Params {
    Unspecified,
    Positional(Vec<DictValue>),
    Named(DictList),
}

#[derive(PartialEq, Clone)]
pub struct Call {
    instance_id: String,
    zome: String,
    function: String,
    params: Params,
}

pub struct CallRpc {
    call: Call,
    id: String,
}

impl From<(Call, u32)> for CallRpc {
    fn from(call_id: (Call, u32)) -> Self {
        CallRpc {
            call: call_id.0,
            id: call_id.1.to_string(),
        }
    }
}

impl From<(Call, String)> for CallRpc {
    fn from(call_id: (Call, String)) -> Self {
        CallRpc {
            call: call_id.0,
            id: call_id.1,
        }
    }
}

impl CallRpc {
    pub fn json(&self) -> String {
        let params = match &self.call.params {
            Params::Unspecified => r#""params":{}"#.to_string(),
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
                                key,
                                if value {
                                    "true"
                                } else {
                                    "false"
                                }
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
        if self.call.instance_id.is_empty() && !self.call.function.is_empty() {
            return format! {
                r#"{{"jsonrpc":"2.0",{},"method":"{}",{}}}"#,
                id, self.call.function, params
            };
        }
        let call = format! {
            r#"{{"instance_id":"{}","zome":"{}","function":"{}",{}}}"#,
            self.call.instance_id, self.call.zome, self.call.function, params
        };
        format! {
            r#"{{"jsonrpc":"2.0",{},"method":"call","params":{}}}"#,
            id, call
        }
    }
}

impl Call {
    pub fn new() -> Self {
        Call {
            instance_id: String::new(),
            zome: String::new(),
            function: String::new(),
            params: Params::Unspecified,
        }
    }

    pub fn clear(&mut self) {
        self.instance_id.clear();
        self.zome.clear();
        self.function.clear();
        self.params = Params::Unspecified;
    }

    pub fn has_function(&self) -> bool {
        !self.function.is_empty()
    }
}

impl Default for Call {
    fn default() -> Self {
        Call::new()
    }
}

// Function method

impl From<String> for Call {
    fn from(function: String) -> Self {
        Call {
            function,
            ..Default::default()
        }
    }
}

impl From<&str> for Call {
    fn from(function: &str) -> Self {
        function.to_string().into()
    }
}

// No param

impl From<&[&str]> for Call {
    fn from(args: &[&str]) -> Self {
        Call {
            instance_id: args[0].into(),
            zome: args[1].into(),
            function: args[2].into(),
            params: Params::Unspecified,
        }
    }
}

// Positional param

impl From<(&[&str], DictValue)> for Call {
    fn from(args: (&[&str], DictValue)) -> Self {
        (args.0, vec![args.1]).into()
    }
}

impl From<(&[&str], &DictValue)> for Call {
    fn from(args: (&[&str], &DictValue)) -> Self {
        (args.0, args.1.clone()).into()
    }
}

// Positional params

impl From<(&[&str], Vec<DictValue>)> for Call {
    fn from(args: (&[&str], Vec<DictValue>)) -> Self {
        Call {
            instance_id: args.0[0].into(),
            zome: args.0[1].into(),
            function: args.0[2].into(),
            params: if args.1.is_empty() {
                Params::Unspecified
            } else {
                Params::Positional(args.1)
            },
        }
    }
}

impl From<(&[&str], &[DictValue])> for Call {
    fn from(args: (&[&str], &[DictValue])) -> Self {
        (args.0, args.1.to_vec()).into()
    }
}

// Named param

impl From<(&[&str], DictItem)> for Call {
    fn from(args: (&[&str], DictItem)) -> Self {
        (args.0, vec![args.1]).into()
    }
}

impl From<(&[&str], &DictItem)> for Call {
    fn from(args: (&[&str], &DictItem)) -> Self {
        (args.0, args.1.clone()).into()
    }
}

// Named params

impl From<(&[&str], DictList)> for Call {
    fn from(args: (&[&str], DictList)) -> Self {
        Call {
            instance_id: args.0[0].into(),
            zome: args.0[1].into(),
            function: args.0[2].into(),
            params: if args.1.is_empty() {
                Params::Unspecified
            } else {
                Params::Named(args.1)
            },
        }
    }
}

impl From<(&[&str], &[DictItem])> for Call {
    fn from(args: (&[&str], &[DictItem])) -> Self {
        (args.0, args.1.to_vec()).into()
    }
}
