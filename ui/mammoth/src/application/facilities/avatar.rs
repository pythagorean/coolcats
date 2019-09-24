use yew::prelude::*;

use mammoth_macros::PropsComponent;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub account: String,
    pub size: u32,
}

#[derive(PropsComponent)]
pub struct Avatar {
    props: Props,
}

impl Renderable<Avatar> for Avatar {
    fn view(&self) -> Html<Self> {
        let _account = &self.props.account;
        let size = self.props.size;

        let class_name = "account__avatar";

        let style = if size == 0 {
            String::new()
        } else {
            format!("width: {}px; height: {}px;", size, size)
        };

        html! {
            <div
                class = class_name,
                style = style
            />
        }
    }
}
