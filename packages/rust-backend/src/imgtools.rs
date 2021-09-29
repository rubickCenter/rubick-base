extern crate scrap;

use base64::encode;
use image::DynamicImage;
use image::{imageops, ImageBuffer, ImageError, Rgb};
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
    if 0 < point && point < limit {
        point
    } else {
        if point == 0 {
            1
        } else {
            limit - 1
        }
    }
}

fn screen_capture_rect_raw(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, ImageError> {
    let mut img = screen_capture_raw();
    let halfw = width / 2;
    let halfh = height / 2;

    // valid top_left
    let top_left_x = if halfw >= x { 1 } else { x - halfw };
    let top_left_y = if halfh >= y { 1 } else { y - halfh };
    let bottom_right_x = x + halfw;
    let bottom_right_y = y + halfw;

    // valid bottom_right
    let bottom_right_x = if img.width() <= bottom_right_x {
        img.width() - 1
    } else {
        bottom_right_x
    };
    let bottom_right_y = if img.height() <= bottom_right_y {
        img.height() - 1
    } else {
        bottom_right_y
    };

    let width = bottom_right_x - top_left_x;
    let height = bottom_right_y - top_left_y;

    let img = imageops::crop(&mut img, top_left_x, top_left_y, width, height);

    Ok(img.to_image())
}

// capture primary screen
// #[allow(dead_code)]
// pub fn screen_capture(path: String) -> Result<(), ImageError> {
//     screen_capture_raw().save_with_format(&path, image::ImageFormat::Png)?;
//     Ok(())
// }

#[allow(dead_code)]
pub fn screen_capture_base64() -> Result<String, ImageError> {
    let img_rgb = DynamicImage::ImageRgb8(screen_capture_raw());
    let mut buf = vec![];
    img_rgb.write_to(&mut buf, image::ImageOutputFormat::Png)?;
    Ok(encode(&buf))
}

// #[allow(dead_code)]
// pub fn screen_capture_rect(
//     x: u32,
//     y: u32,
//     width: u32,
//     height: u32,
//     path: String,
// ) -> Result<(), ImageError> {
//     screen_capture_rect_raw(x, y, width, height)?
//         .save_with_format(path, image::ImageFormat::Png)?;
//     Ok(())
// }

#[allow(dead_code)]
pub fn screen_capture_rect_base64(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<String, ImageError> {
    let img_rgb = DynamicImage::ImageRgb8(screen_capture_rect_raw(x, y, width, height)?);
    let mut buf = vec![];
    img_rgb.write_to(&mut buf, image::ImageOutputFormat::Png)?;
    Ok(encode(&buf))
}

// pick color from picture
// #[allow(dead_code)]
// pub fn color_picker(path: String, x: u32, y: u32) -> Result<Rgba<u8>, ImageError> {
//     let img = image::io::Reader::open(path)?
//         .with_guessed_format()?
//         .decode()?;

//     let x = valid_border(x, img.width());
//     let y = valid_border(y, img.height());
//     let px = img.get_pixel(x, y);
//     Ok(px)
// }

// pick color from primary screen
#[allow(dead_code)]
pub fn screen_color_picker(x: u32, y: u32) -> Result<Rgb<u8>, ImageError> {
    let screen_capture = screen_capture_raw();

    let x = valid_border(x, screen_capture.width());
    let y = valid_border(y, screen_capture.height());
    let px = screen_capture.get_pixel(x, y);
    Ok(*px)
}
