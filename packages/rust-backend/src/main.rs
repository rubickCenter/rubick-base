mod imgtools;

fn main() {
    // imgtools::screen_capture("./cap.png".to_string());
    let a = imgtools::screen_capture_rect_base64(1, 1, 60, 60).unwrap();
    let _s = imgtools::screen_capture_rect(1, 1, 60, 60, "./a.png".to_string()).unwrap();

    println!("{:?}", a);
}
