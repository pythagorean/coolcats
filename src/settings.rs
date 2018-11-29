use yew::prelude::*;

pub struct Settings {
    show: bool,
}

pub enum Msg {
    UpdateHandleText(ChangeData),
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub show: bool,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            show: false,
        }
    }
}

impl Component for Settings {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Settings {
            show: props.show,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateHandleText(ChangeData::Value(handle)) => {
                js! { alert(@{handle}) };
            },
            _ => (),
        };
        true
    }
}

impl Renderable<Settings> for Settings {
    fn view(&self) -> Html<Self> {
        // Render nothing if the "show" prop is false
        if !self.show {
            return html! { </> };
        }
        let use_handle_text = "";
        let handle_taken = false;
        html! {
            <div classname="panel panel-default",>
                <div classname="panel-body",>
                    <div style="padding-left: 30; padding-bottom: 10;",>
                        <p
                            classname="text-info",
                            style={
                                if use_handle_text.len() == 0 && handle_taken == false {
                                    "display: inline;"
                                } else {
                                    "display: none;"
                                }
                            },
                        >
                            {"Set your handle to get meowing"}
                        </p>
                    </div>
                    <div style="padding-left: 30; padding-bottom: 10;",>
                        <p
                            classname="text-danger",
                            style={
                                if handle_taken == true {
                                    "display: inline;"
                                } else {
                                    "display: none;"
                                }
                            },
                        >
                            {"This handle already has a home, try something else!"}
                        </p>
                    </div>
                    <form
                        id="handleForm",
                        classname="form-group",
                    >
                        <div classname="col-xs-8",>
                            <div classname="form-group input-icon",>
                                <i>{"@"}</i>
                                <input
                                    value=use_handle_text,
                                    onchange=|input| Msg::UpdateHandleText(input),
                                    type="text",
                                    classname="form-control",
                                    id="myHandle",
                                    placeholder="handle",
                                />
                            </div>
                        </div>
                        <div classname="col-xs-2",>
                            <button
                                id="setHandleButton",
                                type="submit",
                                classname="btn btn-primary",
                            >
                                {"Set Handle"}
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        }
    }
}
