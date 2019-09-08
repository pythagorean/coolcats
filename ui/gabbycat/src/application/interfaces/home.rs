use yew::prelude::*;

use crate::application::resources;

pub struct Home {
    resources: Box<dyn Bridge<resources::Worker>>,
    values: Vec<String>,
}

pub enum Msg {
    Resources(resources::Response),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let resources = resources::Worker::bridge(link.send_back(Msg::Resources));
        let mut component = Self {
            resources,
            values: Vec::new(),
        };
        component.initialize();
        component
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Resources(response) => match response {
                resources::Response::LocaleValues(values) => {
                    self.values = values;
                    return true;
                }
            },
        }
    }
}

impl Home {
    fn initialize(&mut self) {
        self.resources.send(resources::Request::LocaleValues(vec![
            "compose_form-placeholder".into(),
        ]));
    }
}

impl Renderable<Home> for Home {
    fn view(&self) -> Html<Self> {
        html! {
            <p>{format!("{:?}", self.values)}</p>
        }
    }
}
