// Should adapt functionality from https://github.com/andreypopp/react-textarea-autosize

use yew::prelude::*;

use gabbycat_macros::PropsComponent;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: String,
    pub placeholder: String,
    pub auto_focus: bool,
    pub aria_autocomplete: String,
}

#[derive(PropsComponent)]
pub struct TextareaAutosize {
    props: Props,
}

impl Renderable<TextareaAutosize> for TextareaAutosize {
    fn view(&self) -> Html<Self> {
        let class = &self.props.class[..];
        let placeholder = &self.props.placeholder;
        let autofocus = self.props.auto_focus;
        let aria_autocomplete = &self.props.aria_autocomplete;

        html! {
            <textarea
                class = class,
                placeholder = placeholder,
                autofocus = autofocus,
                aria-autocomplete = aria_autocomplete
            />
        }
    }
}
