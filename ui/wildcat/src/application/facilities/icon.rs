use yew::prelude::*;

use wildcat_macros::{PropsComponent, class_names};

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub id: String,
    pub class: String,
    pub fixed_width: bool,
}

#[derive(PropsComponent)]
pub struct Icon {
    props: Props,
}

impl Renderable<Icon> for Icon {
    fn view(&self) -> Html<Self> {
        let id = &self.props.id;
        let fixed_width = self.props.fixed_width;
        let class_names = format!(
            "fa-{} {} {}",
            id,
            class_names!("fa", "fa-fw" => fixed_width),
            &self.props.class
        );

        html! {
            <i
                role = "img",
                alt = id,
                class = class_names
            />
        }
    }
}
