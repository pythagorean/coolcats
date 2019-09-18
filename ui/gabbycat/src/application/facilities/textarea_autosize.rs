// Should adapt functionality from https://github.com/andreypopp/react-textarea-autosize

use yew::prelude::*;

pub struct TextareaAutosize {
    class: String,
    placeholder: String,
    aria_autocomplete: String,
}

impl Renderable<TextareaAutosize> for TextareaAutosize {
    fn view(&self) -> Html<Self> {
        let class = &self.class[..];
        let placeholder = &self.placeholder;
        let aria_autocomplete = &self.aria_autocomplete;

        html! {
            <textarea
                class = class,
                placeholder = placeholder,
                aria-autocomplete = aria_autocomplete
            />
        }
    }
}

pub enum Msg {}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: String,
    pub placeholder: String,
    pub aria_autocomplete: String,
}

impl Component for TextareaAutosize {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            class: props.class,
            placeholder: props.placeholder,
            aria_autocomplete: props.aria_autocomplete,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.class = props.class;
        self.placeholder = props.placeholder;
        self.aria_autocomplete = props.aria_autocomplete;
        true
    }
}
