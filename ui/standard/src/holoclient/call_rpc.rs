use coolcats_utils::{DictValue, DictItem, DictList};

#[derive(PartialEq, Clone)]
pub enum Args {
    Unspecified,
    Positional(Vec<DictValue>),
    Named(DictList),
}

#[derive(PartialEq, Clone)]
pub struct Call {
    instance_id: String,
    zome: String,
    function: String,
    args: Args,
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
        let args = match &self.call.args {
            Args::Unspecified => r#""args":null"#.to_string(),
            Args::Positional(positional_args) => {
                let mut args = Vec::new();
                for arg in positional_args {
                    match arg {
                        DictValue::String(ref value) => {
                            args.push(format! {
                                r#""{}""#, value
                            });
                        }
                        DictValue::Strings(ref value) => {
                            args.push(format! {
                                r#"["{}"]"#, value.join(r#"",""#)
                            });
                        }
                        DictValue::Integer(value) => {
                            args.push(value.to_string());
                        }
                        DictValue::Bool(value) => {
                            args.push(
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
                    r#""args":[{}]"#,
                    args.join(",")
                }
            }
            Args::Named(named_args) => {
                let mut args = Vec::new();
                for arg in named_args {
                    let key = &arg.0;
                    match arg.1 {
                        DictValue::String(ref value) => {
                            args.push(format! {
                                r#""{}":"{}""#,
                                key, value
                            });
                        }
                        DictValue::Strings(ref value) => {
                            args.push(format! {
                                r#""{}":["{}"]"#,
                                key, value.join(r#"",""#)
                            });
                        }
                        DictValue::Integer(value) => {
                            args.push(format! {
                                r#""{}":{}"#,
                                key, value
                            });
                        }
                        DictValue::Bool(value) => {
                            args.push(format! {
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
                    r#""args":{{{}}}"#,
                    args.join(",")
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
                id, self.call.function, args.replace("args", "params")
            };
        }
        let params = format! {
            r#"{{"instance_id":"{}","zome":"{}","function":"{}",{}}}"#,
            self.call.instance_id, self.call.zome, self.call.function, args
        };
        format! {
            r#"{{"jsonrpc":"2.0",{},"method":"call","params":{}}}"#,
            id, params
        }
    }
}

impl Call {
    pub fn new() -> Self {
        Call {
            instance_id: String::new(),
            zome: String::new(),
            function: String::new(),
            args: Args::Unspecified,
        }
    }

    pub fn clear(&mut self) {
        self.instance_id.clear();
        self.zome.clear();
        self.function.clear();
        self.args = Args::Unspecified;
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

// No arg

impl From<&[&str]> for Call {
    fn from(args: &[&str]) -> Self {
        Call {
            instance_id: args[0].into(),
            zome: args[1].into(),
            function: args[2].into(),
            args: Args::Unspecified,
        }
    }
}

// Positional arg

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

// Positional args

impl From<(&[&str], Vec<DictValue>)> for Call {
    fn from(args: (&[&str], Vec<DictValue>)) -> Self {
        Call {
            instance_id: args.0[0].into(),
            zome: args.0[1].into(),
            function: args.0[2].into(),
            args: if args.1.is_empty() {
                Args::Unspecified
            } else {
                Args::Positional(args.1)
            },
        }
    }
}

impl From<(&[&str], &[DictValue])> for Call {
    fn from(args: (&[&str], &[DictValue])) -> Self {
        (args.0, args.1.to_vec()).into()
    }
}

// Named arg

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

// Named args

impl From<(&[&str], DictList)> for Call {
    fn from(args: (&[&str], DictList)) -> Self {
        Call {
            instance_id: args.0[0].into(),
            zome: args.0[1].into(),
            function: args.0[2].into(),
            args: if args.1.is_empty() {
                Args::Unspecified
            } else {
                Args::Named(args.1)
            },
        }
    }
}

impl From<(&[&str], &[DictItem])> for Call {
    fn from(args: (&[&str], &[DictItem])) -> Self {
        (args.0, args.1.to_vec()).into()
    }
}
