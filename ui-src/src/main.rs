extern crate yew;
extern crate coolcats2;

use yew::prelude::App;
use coolcats2::Model;
use coolcats2::holoclient::HoloclientService;

fn main() {
    HoloclientService::new().connect();
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
