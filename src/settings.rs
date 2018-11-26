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
        html! { <p>{"Hello from Settings"}</p> }
    }
}
