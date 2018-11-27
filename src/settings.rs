use yew::prelude::*;

pub struct Settings {
    show: bool,
}

pub enum Msg {}

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

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Settings> for Settings {
    fn view(&self) -> Html<Self> {
        // Render nothing if the "show" prop is false
        if !self.show {
            return html! { </> };
        }
        html! {
            <div className="panel panel-default",>
                <div className="panel-body",>
                    <div style="padding-left: 30; padding-bottom: 10;",>
                        <p
                            className="text-info",
                            style={"display: inline;"},
                        >
                            {"Set your handle to get meowing"}
                        </p>
                    </div>
                    <div style="padding-left: 30; padding-bottom: 10;",>
                        <p
                            className="text-danger",
                            style={"display: none;"},
                        >
                            {"This handle already has a home, try something else!"}
                        </p>
                    </div>
                    <form id="handleForm", className="form-group",>
                        <div className="col-xs-8",>
                            <div className="form-group input-icon",>
                                <i>{"@"}</i>
                                <input
                                    value={""},
                                    type="text",
                                    className="form-control",
                                    id="myHandle",
                                    placeholder="handle",
                                />
                            </div>
                        </div>
                        <div className="col-xs-2",>
                        <button
                            id="setHandleButton",
                            type="submit",
                            className="btn btn-primary",
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
