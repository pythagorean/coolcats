use yew::prelude::*;

use mammoth_macros::PropsComponent;
use super::textarea_autosize::TextareaAutosize as Textarea;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub placeholder: String,
    #[props(required)]
    pub auto_focus: bool,
}

#[derive(PropsComponent)]
pub struct AutosuggestTextarea {
    props: Props,
}

impl Renderable<AutosuggestTextarea> for AutosuggestTextarea {
    fn view(&self) -> Html<Self> {
        let placeholder = &self.props.placeholder;
        let auto_focus = self.props.auto_focus;

        html! {
            <div class = "autosuggest-textarea">
                <label>
                    <span style = "display: none;">{placeholder}</span>
                    <Textarea
                        class = "autosuggest-textarea__textarea",
                        placeholder = placeholder,
                        auto_focus = auto_focus,
                        aria_autocomplete = "list"
                    />
                </label>
            </div>
        }
    }
}
