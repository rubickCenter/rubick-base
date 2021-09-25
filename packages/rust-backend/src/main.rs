#![feature(path_try_exists)]

mod sysapp;

fn main() {
    // imgtools::screen_capture("./cap.png".to_string());
    let a = sysapp::find_apps(true, None);
    println!("{:?}", a)
}
