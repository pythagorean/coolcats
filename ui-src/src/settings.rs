use yew::prelude::*;

const MAX_HANDLE_LENGTH: usize = 20;

pub struct Settings {
    use_handle_text: String,
}

fn get_first_name() -> Option<String> { Some("".into()) }

fn set_first_name(name: &str) {
    js! { alert("set_first_name: " + @{name}) }
}

fn toggle_modal() {}

pub enum Msg {
    UpdateHandleText(ChangeData),
    OnHandleSubmit,
    Ignore,
}

impl Component for Settings {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Settings {
            use_handle_text: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateHandleText(ChangeData::Value(handle_text)) => {
                self.use_handle_text = handle_text;
                return false;
            },
            Msg::UpdateHandleText(_) => (),

            Msg::OnHandleSubmit => {
                self.on_handle_submit();
            },

            Msg::Ignore => (),
        };
        true
    }
}

impl Settings {
    fn use_handle(&mut self, _handle: &str) {
    }

    fn on_handle_submit(&mut self) {
        // empty string given as input
        if self.use_handle_text.len() == 0 { return };

        // max characters exceeded
        if self.use_handle_text.len() > MAX_HANDLE_LENGTH {
            self.use_handle_text = String::from("");
            return
        }

        self.use_handle("abc");

        //self.use_handle(self.use_handle_text);

        // check if a name has been set, and if not default to handle
        match get_first_name() {
            Some(ref first_name) if first_name.len() > 1 => (),
            _ => set_first_name(&self.use_handle_text),
        };

        toggle_modal();
    }
}

impl Renderable<Settings> for Settings {
    fn view(&self) -> Html<Self> {
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
                                    else { Msg::Ignore }
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
