use titlecase::titlecase;
use yew::prelude::*;

pub struct SimpleForm {
    pub form_for: String,
}

impl SimpleForm {
    pub fn new(form_for: &str) -> Self {
        Self {
            form_for: form_for.into(),
        }
    }

    pub fn button<T: Component>(&self, name: &str, text: &str, button_type: &str) -> Html<T> {
        html! {
            <button name = name, type = button_type, class = "btn">
                {text}
            </button>
        }
    }

    pub fn input_boolean<T: Component>(
        &self,
        name: &str,
        hint: &str,
        recommended: bool,
    ) -> Html<T> {
        let (name, render) = if name.contains(':') {
            let v: Vec<&str> = name.splitn(2, ':').collect();
            (v[0], v[1])
        } else {
            (name, name)
        };
        let form_for = self.form_for.clone() + "_" + name;
        let input_class = "boolean optional";
        html! {
            <div class = format!("input with_label {} {} field_with_hint", input_class, form_for)>
                <div class = "label_input">
                    <label class = input_class, for = form_for>
                        {titlecase(render).replace("_", " ")}
                        {if recommended { html! {
                            <span class = "recommended">
                                {"Recommended"}
                            </span>
                        }} else { html! {} }}
                    </label>
                    <div class = "label_input__wrapper">
                        <input
                            value = "0",
                            name = format!("{}[{}]", self.form_for, name),
                            type = "hidden"
                        />
                        <label class = "checkbox">
                            <input
                                value = "1",
                                name = format!("{}[{}]", self.form_for, name),
                                id = form_for,
                                class = input_class,
                                type = "checkbox"
                            />
                        </label>
                    </div>
                </div>
                <span class = "hint">
                    {hint}
                </span>
            </div>
        }
    }

    pub fn input_file<T: Component>(&self, name: &str, accept: &str, hint: &str) -> Html<T> {
        let form_for = self.form_for.clone() + "_" + name;
        let input_class = "file optional";

        html! {
            <div class = format!("input with_label file optional {} {} field_with_hint", input_class, form_for)>
                <div class = "label_input">
                    <label class = input_class, for = form_for>
                        {titlecase(name).replace("_", " ")}
                    </label>
                    <div class = "label_input__wrapper">
                        <input
                            accept = accept,
                            class = input_class,
                            type = "file",
                            name = format!("{}[{}]", self.form_for, name),
                            id = form_for
                        />
                    </div>
                </div>
                <span class = "hint">{hint}</span>
            </div>
        }
    }

    pub fn input<T: Component>(&self, name: &str, maxlength: u16) -> Html<T> {
        let (name, render) = if name.contains(':') {
            let v: Vec<&str> = name.splitn(2, ':').collect();
            (v[0], v[1])
        } else {
            (name, name)
        };
        let form_for = self.form_for.clone() + "_" + name;
        let input_class = if maxlength < 500 {
            "string optional"
        } else {
            "text optional"
        };

        html! {
            <div class = format!("input with_label {} {}", input_class, form_for)>
                <div class = "label_input">
                    <label class = input_class, for = form_for>
                        {titlecase(render).replace("_", " ")}
                    </label>
                    <div class = "label_input__wrapper">
                        {if maxlength < 500 { html! {
                            <input
                                name = format!("{}[{}]", self.form_for, name),
                                id = form_for,
                                maxlength = maxlength,
                                class = input_class,
                                size = maxlength,
                                type = "text"
                            />
                        }} else { html! {
                            <textarea
                                name = format!("{}[{}]", self.form_for, name),
                                id = form_for,
                                maxlength = maxlength,
                                class = input_class,
                                type = "text"
                            />
                        }}}
                    </div>
                </div>
            </div>
        }
    }

    pub fn fields_input<T: Component>(
        &self,
        fields_for: u32,
        name: &str,
        placeholder: &str,
    ) -> Html<T> {
        let form_for = &self.form_for;

        html! {
            <div class = format!("input string optional {}_fields_{}", form_for, name)>
                <input
                    maxlength = 255,
                    class = "string optional",
                    placeholder = placeholder,
                    size = 255,
                    type = "text",
                    value = "",
                    name = format!("{}[fields_attributes][{}][{}]", form_for, fields_for, name),
                    id = format!("{}_fields_attributes_{}_{}", form_for, fields_for, name)
                />
            </div>
        }
    }
}
