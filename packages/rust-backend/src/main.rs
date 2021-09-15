mod imgtools;

fn main() {
    // imgtools::screen_capture("./cap.png".to_string());
    let _a = imgtools::screen_capture_rect(10, 30, 60, 60, "./c.png".to_string());
    // println!("{:?}", a);
}
