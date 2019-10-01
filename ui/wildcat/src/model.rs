use yew::{prelude::*, html::Scope};

use coolcats_ui_shared::holoclient::{self, Holoclient, ToHoloclient, ToApplication};
use crate::application::{self, Application};

pub enum ModelType {
    Holoclient,
    Application,
}

pub struct Model {
    model_type: Option<ModelType>,
    partner: Option<Scope<Model>>,
    params: Box<dyn Params>,
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        match &self.model_type {
            Some(model_type) => match model_type {
                ModelType::Holoclient => html! {
                    <Holoclient
                        params = self.params.holoclient(),
                        callback = Msg::FromHoloclient
                    />
                },
                ModelType::Application => html! {
                    <Application
                        params = self.params.application(),
                        callback = Msg::FromApplication
                    />
                },
            },
            None => html! {},
        }
    }
}

trait Params {
    fn holoclient(&self) -> holoclient::Params {
        holoclient::Params::new()
    }

    fn application(&self) -> application::Params {
        application::Params(ToApplication::None)
    }
}

struct NoParams;
impl Params for NoParams {}

struct HoloclientParams(holoclient::Params);
impl Params for HoloclientParams {
    fn holoclient(&self) -> holoclient::Params {
        self.0.clone()
    }
}

struct ApplicationParams(application::Params);
impl Params for ApplicationParams {
    fn application(&self) -> application::Params {
        self.0.clone()
    }
}

pub enum Msg {
    SetModel(ModelType, Scope<Model>),
    FromApplication(ToHoloclient),
    ToHoloclient(ToHoloclient),
    FromHoloclient(ToApplication),
    ToApplication(ToApplication),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            model_type: None,
            partner: None,
            params: Box::new(NoParams),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetModel(model_type, partner) => {
                self.model_type = Some(model_type);
                self.partner = Some(partner);
            }
            Msg::FromApplication(msg) => {
                self.params = Box::new(NoParams);
                self.partner.as_mut().unwrap().send_message(Msg::ToHoloclient(msg));
            }
            Msg::ToHoloclient(params_from_application) => {
                let ToHoloclient::Call(params) = params_from_application;
                self.params = Box::new(HoloclientParams(params));
            }
            Msg::FromHoloclient(msg) => {
                self.params = Box::new(NoParams);
                self.partner.as_mut().unwrap().send_message(Msg::ToApplication(msg));
            }
            Msg::ToApplication(params_from_holoclient) => {
                let params = application::Params(params_from_holoclient);
                self.params = Box::new(ApplicationParams(params));
            }
        }
        true
    }
}
