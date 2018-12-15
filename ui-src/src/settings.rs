use yew::prelude::*;

const MAX_HANDLE_LENGTH: usize = 20;

pub struct Settings {
    show: bool,
    use_handle_text: String,
}

fn use_handle(_handle_text: &str) {}
fn get_first_name() -> Option<String> { Some("".into()) }
fn set_first_name(name: &str) {
    js! { alert("set_first_name: " + @{name}) }
}
fn toggle_modal() {}

impl Settings {
    fn on_handle_submit(&mut self) {
        let use_handle_text = &self.use_handle_text;

        // empty string given as input
        if use_handle_text.len() == 0 { return };

        // max characters exceeded
        if use_handle_text.len() > MAX_HANDLE_LENGTH {
            self.use_handle_text = String::from("");
            return
        }

        use_handle(use_handle_text);

        // check if a name has been set, and if not default to handle
        match get_first_name() {
            Some(ref first_name) if first_name.len() > 1 => (),
            _ => set_first_name(use_handle_text),
        };

        toggle_modal();
    }
}

pub enum Msg {
    UpdateHandleText(ChangeData),
    OnHandleSubmit,
    Nope,
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
            use_handle_text: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateHandleText(ChangeData::Value(handle_text)) => {
                self.use_handle_text = handle_text;
                return false;
            },
            Msg::OnHandleSubmit => {
                self.on_handle_submit();
            }
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
        let use_handle_text = &self.use_handle_text;
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
                    <div classname="col-xs-8",>
                        <div classname="form-group input-icon",>
                            <i>{"@"}</i>
                            <input
                                value=use_handle_text,
                                onchange=|input| Msg::UpdateHandleText(input),
                                onkeypress=|pressed| {
                                    if pressed.key() == "Enter" { Msg::OnHandleSubmit }
                                    else { Msg::Nope }
                                },
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
                            classname="btn btn-primary",
                            onclick=|_| Msg::OnHandleSubmit,
                        >
                            {"Set Handle"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
