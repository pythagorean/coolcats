pub struct Redux {}

impl Redux {
    pub fn create(call_response: &str) {
        js! {
            alert(@{
                format!("Redux create {}", call_response)
            });
            console.log(hcMw);
        }
    }
}
