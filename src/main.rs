extern crate yew;
extern crate coolcats2;

use yew::prelude::App;
use coolcats2::AppModel;

fn main() {
    yew::initialize();
    App::<AppModel>::new().mount_to_body();
    yew::run_loop();
}
