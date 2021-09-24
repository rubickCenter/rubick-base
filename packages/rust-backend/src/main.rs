#![feature(path_try_exists)]
use std::io::Write;

mod sysapp;

fn main() {
    // imgtools::screen_capture("./cap.png".to_string());
    let a = sysapp::find_apps();
    std::fs::File::create("out.json")
        .unwrap()
        .write(a.as_bytes())
        .unwrap();
}
