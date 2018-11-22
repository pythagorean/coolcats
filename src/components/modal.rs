use yew::prelude::*;

pub struct Modal {
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

impl Component for Modal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Modal {
            show: props.show,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Modal> for Modal {
    fn view(&self) -> Html<Self> {
        // Render nothing if the "show" prop is false
        if !self.show {
            return html! { <div /> };
        }

        // The gray background
        let backdrop_style = "\
            position: fixed; \
            top: 0; \
            bottom: 0; \
            left: 0; \
            right: 0; \
            background-color: rgba(0,0,0,0.3); \
            padding: 50;";

        // The modal "window"
        let modal_style = "\
            background-color: #fff; \
            borderRadius: 5; \
            maxWidth: 500; \
            minHeight: 200; \
            margin: 0 auto; \
            padding: 30;";

        html! {
            <div style={backdrop_style},>
                <div style={modal_style},>
                    <div align="center",>
                        <p classname="h1",>{"Welcome to Coolcats2!"}</p>
                    </div>
                </div>
            </div>
        }
    }
}
