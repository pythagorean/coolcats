extern crate yew;
extern crate coolcats2;

use yew::prelude::App;
use coolcats2::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
