use yew::prelude::*;
use stdweb::web::Date;

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

impl Renderable<Meow> for Meow {
    fn view(&self) -> Html<Self> {
        let stamp = &self.post.string("stamp");
        let message = &self.post.string("message");
        let _author = &self.post.string("author");
        let _address = &self.post.string("address");
        let user_handle = &self.post.string("user_handle");
        html! {<>
            <div class="meow", id={stamp},>
                <a class="meow-edit", href="#",>
                    {"edit"}
                </a>
                <a class="user", href="#",>
                    {"@"}{user_handle}
                </a>
                {" | "}
                <a class="stamp", href="#",>
                    { Date::from_time(stamp.parse().unwrap()).to_string() }
                </a>
                <div class="message",>{message}</div>
                /*
                <a className="meow-edit" onClick={() => "openEditPost('+id+')"}>
                  edit
                </a>
                <Link to={`/u/${author}`} className="user">
                  @{userHandle}
                </Link>{' '}
                |{' '}
                <Link to={`/meow/${hash}`} className="stamp">
                  {new Date(stamp).toString()}
                </Link>
                <div className="message">{this.urlify(message)}</div>
                <FavesContainer hash={hash} />
                */
            </div>
        </>}
    }
}
