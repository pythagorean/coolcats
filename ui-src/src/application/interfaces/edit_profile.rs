use stdweb::{web::File, unstable::TryInto};

use yew::{
    prelude::*,
    services::{DialogService, ReaderService, Task},
};

use crate::application::{
    Action,
    context::{self, ContextAgent},
    reader::ReaderServiceExt,
    state::State,
};

const MAX_PIC_SIZE: u32 = 2_000_000;

interface_getstates!("handle", "profile_pic", "first_name");
interface_component!(EditProfile);

// This will be mapped to EditProfile.local:
pub struct Local {
    new_name_text: String,
    new_profile_pic: String,
    reader: ReaderService,
    reader_job: Option<Box<Task>>,
}

impl Local {
    fn new() -> Self {
        Self {
            new_name_text: String::new(),
            new_profile_pic: String::new(),
            reader: ReaderService::new(),
            reader_job: None,
        }
    }
}

pub enum LocalMsg {
    NewStates,
    UpdateNameText(InputData),
    LoadImage,
    LoadedImage(String),
    OnSubmit,
    Ignore,
}

impl EditProfile {
    fn local_update(&mut self, local_msg: LocalMsg) -> ShouldRender {
        match local_msg {
            LocalMsg::NewStates => {
                self.local.new_name_text = self.getstate.string("first_name").clone();
                return true;
            }

            LocalMsg::UpdateNameText(input) => {
                self.local.new_name_text = input.value;
                return true;
            }

            LocalMsg::LoadImage => {
                self.load_image();
            }

            LocalMsg::LoadedImage(dataurl) => {
                self.loaded_image(dataurl);
            }

            LocalMsg::OnSubmit => {
                self.on_submit();
            }

            LocalMsg::Ignore => (),
        };
        false
    }

    fn set_profile_pic(&mut self, profile_pic: &str) {
        self.update(Action::SetProfilePic(profile_pic.into()).into());
    }

    fn set_first_name(&mut self, name: &str) {
        self.update(Action::SetFirstName(name.into()).into());
    }

    fn on_submit(&mut self) {
        let new_profile_pic = self.local.new_profile_pic.clone();
        if !new_profile_pic.is_empty() && new_profile_pic != *self.getstate.string("profile_pic") {
            self.set_profile_pic(&new_profile_pic);
        }

        let new_name_text = self.local.new_name_text.clone();
        if !new_name_text.is_empty() && new_name_text != *self.getstate.string("first_name") {
            self.set_first_name(&new_name_text);
        }

        self.update(Action::Redirect("/#/".into()).into());
    }

    fn load_image(&mut self) {
        let mut dialog = DialogService::new();
        let file = js! { return document.querySelector("#image").files[0] };
        let file_size: u32 = js! { return @{file.clone()}.size }.try_into().unwrap();
        if file_size > MAX_PIC_SIZE {
            dialog.alert("File is too big!");
            return;
        }
        let file: File = file.try_into().unwrap();
        let callback = self.link.send_back(|dataurl| LocalMsg::LoadedImage(dataurl).into());
        let reader_job = self.local.reader.read_file_as_dataurl(&file, callback);
        self.local.reader_job = Some(Box::new(reader_job));
    }

    fn loaded_image(&mut self, dataurl: String) {
        self.local.new_profile_pic = dataurl;
        self.local.reader_job = None;
    }
}

impl Renderable<EditProfile> for EditProfile {
    fn view(&self) -> Html<Self> {
        if self.getstate.is_empty() {
            return html! { <></> };
        };
        let handle = &self.getstate.string("handle");
        let new_name_text = &self.local.new_name_text;

        html! {
            <div class="panel panel-default",>
                <div class="close",>
                    <a href="/#/",>{"x"}</a>
                </div>
                <div class="panel-body",>
                    <p>{"Profile"}</p>
                    <div class="form-row",>
                        <div class="form-group col-xs-6",>
                            <label>{"Handle"}</label>
                            <p id="handle",>{"@"}{handle}</p>
                        </div>
                        <div class="form-group col-xs-6",>
                            <label>{"Name"}</label>
                            <input
                                type="text",
                                class="form-control",
                                id="inputName",
                                placeholder="name",
                                value={new_name_text},
                                oninput=|input| LocalMsg::UpdateNameText(input).into(),
                                onkeypress=|pressed| {
                                    if pressed.key() == "Enter" { LocalMsg::OnSubmit.into() }
                                    else { LocalMsg::Ignore.into() }
                                },
                            />
                        </div>
                        <div class="form-group",>
                            <div class="form-group col-xs-10",>
                                <label>{"Profile Picture"}</label>
                                <input
                                    type="file",
                                    accept="image/*",
                                    id="image",
                                    hidden=true,
                                    oninput=|_| LocalMsg::LoadImage.into(),
                                />
                            </div>
                        </div>
                    </div>
                    <div class="form-group col-xs-6",>
                        <button
                            id="saveChanges",
                            class="btn btn-primary",
                            onclick=|_| LocalMsg::OnSubmit.into(),
                        >
                            {"Save Changes"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
