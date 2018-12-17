use std::process::Command;

fn main() {
    Command::new("yarn").args(&["run", "webpack"])
            .spawn().expect("webpack failed to start");
}
