use yew::prelude::*;

pub struct AutosuggestTextarea {
    placeholder: String,
}

impl Renderable<AutosuggestTextarea> for AutosuggestTextarea {
    fn view(&self) -> Html<Self> {
        let placeholder = &self.placeholder;

        html! {
            <div class="compose-form__autosuggest-wrapper", key="compose-form__autosuggest-wrapper">
                <div class="autosuggest-textarea">
                    <label>
                        <span /*style="display: none;"*/>{placeholder}</span>
                        //<Textarea placeholder = placeholder/>
                    </label>
                </div>
            </div>
        }
    }
}

pub enum Msg {}

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub placeholder: String,
}

impl Component for AutosuggestTextarea {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            placeholder: props.placeholder,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.placeholder = props.placeholder;
        true
    }
}
