extern crate scrap;

use image::{GenericImageView, ImageError, Rgba};
use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
extern crate image;

#[allow(dead_code)]
pub fn screen_capture(path: String) {
    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        // Wait until there's a frame.
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };
        let stride = buffer.len() / h;
        let mut imgbuf = image::ImageBuffer::new(w as u32, h as u32);
        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let i: usize = stride * y as usize + 4 * x as usize;
            *pixel = image::Rgb([buffer[i + 2], buffer[i + 1], buffer[i]]);
        }

        imgbuf
            .save_with_format(path, image::ImageFormat::Png)
            .expect("img save error!");
        break;
    }
}

#[allow(dead_code)]
pub fn color_picker(path: String, x: u32, y: u32) -> Result<Rgba<u8>, ImageError> {
    let img = image::io::Reader::open(path)?
        .with_guessed_format()?
        .decode()?;
    let px = img.get_pixel(x, y);
    Ok(px)
}
