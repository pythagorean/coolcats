use yew::prelude::*;
use stdweb::web::Date;
use regex::Regex;

use crate::utils::Dict;

pub struct Meow {
    pub post: Dict,
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub post: Dict,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            post: Dict::new(),
        }
    }
}

impl Component for Meow {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Meow {
            post: props.post,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.post = props.post;
        true
    }
}

impl Meow {
    // replace URLs with links
    fn urlify(&self, text: &str) -> Html<Self> {
        let re = Regex::new(r"(https?://[^\\s]+)").unwrap();
        let mut matches = 0;
        let v: Vec<Html<Self>> = text
            .split_whitespace()
            .map(|s| {
                if re.is_match(s) {
                    matches += 1;
                    html! {<>
                        <a
                            key={matches},
                            target="_blank",
                            href={s},
                        >
                            {s}
                        </a>
                        {' '}
                    </>}
                } else if s.contains('#') {
                    self.hashify(s)
                } else {
                    html! {<>{s}{' '}</>}
                }
            })
            .collect();
        html! {<>{for v}{' '}</>}
    }

    //identify all hashtags and replace with links
    fn hashify(&self, text: &str) -> Html<Self> {
        let re = Regex::new(r"(\B#\w*[a-zA-Z]+\w*)").unwrap();
        let mut matches = 0;
        let v: Vec<Html<Self>> = text
            .split_whitespace()
            .map(|s| {
                if re.is_match(s) {
                    matches += 1;
                    html! {<>
                        <a
                            key={matches},
                            href={format!("/#/tag/{}", s.replace("#", ""))},
                            class="hashtag",
                        >
                            {s}
                        </a>
                        {' '}
                    </>}
                } else {
                    html! {<>{s}{' '}</>}
                }
            })
            .collect();
        html! {<>{for v}{' '}</>}
    }
}

impl Renderable<Meow> for Meow {
    fn view(&self) -> Html<Self> {
        let stamp = &self.post.string("stamp");
        let message = &self.post.string("message");
        let author = &self.post.string("author");
        let address = &self.post.string("address");
        let user_handle = &self.post.string("user_handle");
        html! {<>
            <div class="meow", id={stamp},>
                <a class="meow-edit", href="#",>
                    {"edit"}
                </a>
                /*
                <a className="meow-edit" onClick={() => "openEditPost('+id+')"}>
                  edit
                </a>
                */
                <a class="user", href={format!("/#/u/{}", author)},>
                    {"@"}{user_handle}
                </a>
                {" | "}
                <a class="stamp", href={format!("/#/meow/{}", address)},>
                    { Date::from_time(stamp.parse().unwrap()).to_string() }
                </a>
                <div class="message",>{self.urlify(message)}</div>
                /*
                <FavesContainer hash={hash} />
                */
            </div>
        </>}
    }
}
