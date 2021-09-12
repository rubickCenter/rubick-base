mod imgtools;

fn main() {
    // imgtools::screen_capture("./cap.png".to_string());
    let a = imgtools::color_picker("./cap.png".to_string(), 44, 44).unwrap();
    println!("{:?}", a);
}
