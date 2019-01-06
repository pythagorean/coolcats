pub struct WsRpc {
    method: String,
    params: Vec<(String, String)>, // currently only handling first vec element
    id: u32,
}

impl WsRpc {
    pub fn json(&self) -> String {
        let method = format! {
            r#""method":"{}""#,
            self.method
        };
        let params: String;
        if self.params.is_empty() {
            params = r#""params": null"#.into();
        } else {
            params = format! {
                r#""params":{{"{}":"{}"}}"#,
                self.params[0].0, self.params[0].1
            };
        }
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

#[derive(PartialEq, Clone, Debug)]
pub struct Call {
    method: String,
    params: Vec<(String, String)>,
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
                false => vec![((args.1[0]).0.into(), (args.1[0]).1.into())],
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
