macro_rules! interface_getstates {
    ($($x: expr),+) => {
        pub fn getstates() -> Vec<String> {
            lazy_static! {
                static ref VS: Vec<String> = {
                    let mut vector = Vec::new();
                    $(vector.push($x.to_string());)+
                    vector
                };
            }
            VS.to_vec()
        }
    };
}

macro_rules! interface_view_only {
    ($name:ident) => {
        pub struct Local;
        impl Local {
            fn new() -> Self {
                Self
            }
        }
        pub enum LocalMsg {
            NewStates,
        }

        impl $name {
            fn local_update(&self, _local_msg: LocalMsg) -> ShouldRender {
                false
            }
        }
    };
}

macro_rules! interface_component {
    ($name:ident) => {
        interface_component!($name, counter, u32, 0);
    };
    ($name:ident, $prop:ident, $prop_type:ty, $prop_dflt:expr) => {
        #[allow(dead_code)]
        pub enum Msg {
            Action(Action),
            ContextMsg(context::Response),
            GetStates,
            Local(LocalMsg),
        }

        impl From<Action> for Msg {
            fn from(action: Action) -> Self {
                Msg::Action(action)
            }
        }

        impl From<LocalMsg> for Msg {
            fn from(msg: LocalMsg) -> Self {
                Msg::Local(msg)
            }
        }

        #[allow(dead_code)]
        pub struct $name {
            context: Box<Bridge<ContextAgent>>,
            link: ComponentLink<$name>,
            getstate: State,
            local: Local,
            $prop: $prop_type,
        }

        #[derive(PartialEq, Clone)]
        pub struct Props {
            pub $prop: $prop_type,
        }

        impl Default for Props {
            fn default() -> Self {
                Props {
                    $prop: $prop_dflt,
                }
            }
        }

        impl Component for $name {
            type Message = Msg;
            type Properties = Props;

            fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
                let context = ContextAgent::bridge(link.send_back(Msg::ContextMsg));
                let mut component = Self {
                    context,
                    link,
                    getstate: State::unset(),
                    local: Local::new(),
                    $prop: props.$prop,
                };
                component.update(Msg::GetStates);
                component
            }

            fn update(&mut self, msg: Self::Message) -> ShouldRender {
                match msg {
                    Msg::GetStates => {
                        self.context.send(context::Request::GetStates(getstates()));
                    }

                    Msg::Action(action) => {
                        self.context.send(context::Request::Action(action));
                    }

                    Msg::ContextMsg(response) => match response {
                        context::Response::GetStates(getstate) => {
                            if self.getstate != getstate {
                                self.getstate = getstate;
                                self.update(LocalMsg::NewStates.into());
                                return true;
                            }
                        }

                        context::Response::Request(_, _) => (),
                    },

                    Msg::Local(msg) => {
                        return self.local_update(msg);
                    }
                };
                false
            }

            fn change(&mut self, props: Self::Properties) -> ShouldRender {
                self.$prop = props.$prop;
                self.update(Msg::GetStates);
                true
            }
        }
    };
}
