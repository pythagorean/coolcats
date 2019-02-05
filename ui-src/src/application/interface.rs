use crate::application::{ Action, context };

pub enum Msg {
    Action(Action),
    ContextMsg(context::Response),
    GetStates,
}

impl From<Action> for Msg {
    fn from(action: Action) -> Self {
        Msg::Action(action)
    }
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

macro_rules! impl_interface_component {
    ($($t:ty),+) => {
        $(impl Component for $t {
            type Message = Msg;
            type Properties = Props;

            fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
                let context = ContextAgent::bridge(link.send_back(Msg::ContextMsg));
                let mut component = Self {
                    context,
                    getstate: State::unset(),
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
                };
                false
            }

            fn change(&mut self, _: Self::Properties) -> ShouldRender {
                self.update(Msg::GetStates);
                false
            }
        })+
    }
}
