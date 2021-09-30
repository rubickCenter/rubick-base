#![feature(path_try_exists)]
#![allow(dead_code)]

mod asar;
mod imgtools;
mod ioio;
mod sysapp;

fn sysapp() {
    let a = sysapp::find_apps(true, None);
    println!("{:?}", a)
}

fn main() {
    asar::extract(
        "/home/sovlookup/桌面/新建文件夹/a.asar",
        "/home/sovlookup/桌面/新建文件夹/output",
    )
    .unwrap();
    // ioio::send("Mouse", "Wheel", &ioio::Info::Button("Down".to_string()))
    // println!("{:?}", imgtools::get_all_screens())
}
