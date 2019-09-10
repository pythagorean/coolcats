use yew::prelude::*;

pub struct UploadForm;

impl Renderable<UploadForm> for UploadForm {
    fn view(&self) -> Html<Self> {
        let media_ids = String::new();

        html! {
            <div class = "compose-form__upload-wrapper">
                /*<UploadProgressContainer />*/

                <div class = "compose-form__uploads-wrapper">
                    /*
                    {mediaIds.map(id => (
                      <UploadContainer id={id} key={id} />
                    ))}
                    */
                </div>

                {if !media_ids.is_empty() { html! {/*<SensitiveButton />*/}} else { html! {} }}
            </div>
        }
    }
}

pub enum Msg {}

impl Component for UploadForm {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}
