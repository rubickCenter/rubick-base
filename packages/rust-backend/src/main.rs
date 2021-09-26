#![feature(path_try_exists)]
#![allow(dead_code)]

mod ioio;
mod sysapp;

fn sysapp() {
    let a = sysapp::find_apps(true, None);
    println!("{:?}", a)
}

fn main() {
    ioio::send("Mouse", "Wheel", &ioio::Info::Button("Down".to_string()))
}
