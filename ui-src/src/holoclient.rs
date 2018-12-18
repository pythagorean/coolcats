use stdweb::Value;
use stdweb::unstable::TryInto;

pub struct HoloclientService(Option<Value>);

impl HoloclientService {
    pub fn new() -> Self {
        let lib = js! {
            return holoclient;
        };
        HoloclientService(Some(lib))
    }

    pub fn connect(&mut self) {
        let lib = self.0.as_ref().expect("holoclient object lost");
        let v: Value = js! {
            var holoclient = @{lib};
            return holoclient.connect;
        };
        let v: Value = v.try_into().expect("cannot connect");
        js! {
            console.log(@{&v});
        };
    }
}
