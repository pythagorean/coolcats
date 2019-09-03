use fluent::{FluentBundle, FluentResource};
use unic_langid::langid;
use yew::prelude::*;

pub struct Root;

pub enum Msg {}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Root {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Root> for Root {
    fn view(&self) -> Html<Self> {
        let ftl_string = "hello-world = Hello, world!".to_string();
        let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

        let langid_en = langid!("en-US");
        let mut bundle = FluentBundle::new(&[langid_en]);

        bundle
            .add_resource(&res)
            .expect("Failed to add FTL resources to the bundle.");

        let msg = bundle
            .get_message("hello-world")
            .expect("Message doesn't exist.");
        let pattern = msg.value.expect("Message has no value.");

        let mut errors = vec![];
        let value = bundle.format_pattern(&pattern, None, &mut errors);

        html! {
            <p>{value}</p>
        }
    }
}
