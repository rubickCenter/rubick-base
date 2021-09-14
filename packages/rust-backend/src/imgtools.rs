extern crate scrap;

use image::{GenericImageView, ImageBuffer, ImageError, Rgb, Rgba};
use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
extern crate image;

// capture primary screen return image raw
fn screen_capture_raw() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
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

        return imgbuf;
    }
}

fn valid_border(point: u32, limit: u32) -> u32 {
    if 0 < point && point <= limit {
        point
    } else {
        limit
    }
}

// capture primary screen
#[allow(dead_code)]
pub fn screen_capture(path: String) {
    screen_capture_raw()
        .save_with_format(path, image::ImageFormat::Png)
        .expect("img save error!");
}

// pick color from picture
#[allow(dead_code)]
pub fn color_picker(path: String, x: u32, y: u32) -> Result<Rgba<u8>, ImageError> {
    let img = image::io::Reader::open(path)?
        .with_guessed_format()?
        .decode()?;

    let x = valid_border(x, img.width());
    let y = valid_border(y, img.height());
    let px = img.get_pixel(x, y);
    Ok(px)
}

// pick color from primary screen
#[allow(dead_code)]
pub fn screen_color_picker(x: u32, y: u32) -> Result<Rgb<u8>, ImageError> {
    let screen_capture = screen_capture_raw();

    let x = valid_border(x, screen_capture.width());
    let y = valid_border(y, screen_capture.height());
    let px = screen_capture.get_pixel(x, y);
    Ok(*px)
}
