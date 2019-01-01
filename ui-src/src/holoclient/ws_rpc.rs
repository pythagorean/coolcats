#[derive(Serialize, Debug)]
pub struct WsRpc {
    jsonrpc: String,
    method: String,
    params: Vec<String>,
    id: u32,
}

impl WsRpc {
    pub fn has_params(&self) -> bool {
        !self.params.is_empty()
    }
}

#[derive(Serialize, Debug)]
pub struct WsRpcNoParams {
    jsonrpc: String,
    method: String,
    params: Option<String>,
    id: u32,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Call {
    method: String,
    params: Vec<String>,
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

impl From<(&str, Vec<String>)> for Call {
    fn from(args: (&str, Vec<String>)) -> Self {
        Call {
            method: args.0.into(),
            params: args.1,
        }
    }
}

impl From<Vec<String>> for Call {
    fn from(method: Vec<String>) -> Self {
        Call {
            method: method.join("/"),
            params: Vec::new(),
        }
    }
}

impl From<(Vec<String>, Vec<String>)> for Call {
    fn from(vecs: (Vec<String>, Vec<String>)) -> Self {
        Call {
            method: vecs.0.join("/"),
            params: vecs.1,
        }
    }
}

impl From<(Call, u32)> for WsRpc {
    fn from(call_id: (Call, u32)) -> Self {
        WsRpc {
            jsonrpc: "2.0".into(),
            method: call_id.0.method,
            params: call_id.0.params,
            id: call_id.1,
        }
    }
}

impl From<WsRpc> for WsRpcNoParams {
    fn from(rpc: WsRpc) -> Self {
        WsRpcNoParams {
            jsonrpc: rpc.jsonrpc,
            method: rpc.method,
            params: None,
            id: rpc.id,
        }
    }
}
