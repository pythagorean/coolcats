macro_rules! interface_view_only {
    ($name:ident) => {
        pub struct Local;
        impl Local {
            fn new() -> Self {
                Local
            }
        }
        pub enum LocalMsg {}

        impl $name {
            fn local_update(&self, _msg: LocalMsg) -> ShouldRender {
                false
            }
        }
    };
}

macro_rules! interface_component {
    ($name:ident) => {
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
            getstate: State,
            local: Local,
        }

        #[derive(PartialEq, Clone)]
        pub struct Props {
            pub counter: u32,
        }

        impl Default for Props {
            fn default() -> Self {
                Props {
                    counter: 0,
                }
            }
        }

        impl Component for $name {
            type Message = Msg;
            type Properties = Props;

            fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
                let context = ContextAgent::bridge(link.send_back(Msg::ContextMsg));
                let mut component = Self {
                    context,
                    getstate: State::unset(),
                    local: Local::new(),
                };
                component.update(Msg::GetStates);
                component
            }

            fn update(&mut self, msg: Self::Message) -> ShouldRender {
                match msg {
                    Msg::GetStates => {
                        self.context.send(context::Request::GetStates(getstates()));
                    }

                    Msg::Action(msg) => {
                        self.context.send(context::Request::Action(msg));
                    }

                    Msg::ContextMsg(response) => match response {
                        context::Response::GetStates(getstate) => {
                            if self.getstate != getstate {
                                self.getstate = getstate;
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

            fn change(&mut self, _: Self::Properties) -> ShouldRender {
                self.update(Msg::GetStates);
                false
            }
        }
    };
}
