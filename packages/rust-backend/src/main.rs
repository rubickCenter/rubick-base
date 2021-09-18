mod sysapp;

fn main() {
    // imgtools::screen_capture("./cap.png".to_string());
    let a = sysapp::find_apps();
    println!("{:#?}", a);
}
