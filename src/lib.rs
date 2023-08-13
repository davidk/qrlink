/// qrlink
/// A crate to transform a link into a scannable QR code
extern crate image;
extern crate qrcodegen;
mod exporters;

/// URL QR code generator
use std::error;

use image::{ImageBuffer, LumaA};
use qrcodegen::{QrCode, QrCodeEcc};

use crate::exporters::methods::{
    make_image as make_image_export, save_image as save_image_export,
    to_svg_string as to_svg_string_export,
};

pub fn encode(url: Option<&str>) -> Result<QrCode, Box<dyn error::Error>> {
    match QrCode::encode_text(url.unwrap(), QrCodeEcc::Low) {
        Ok(qr) => Ok(qr),
        Err(e) => Err(e.into()),
    }
}

/// generates a qrcode that is printed to a terminal/console for quick scanning
/// parameters:
/// - qrcode: encoded qrcode
/// - quiet_zone: the border size to apply to the QR code (created with ASCII_BL_BLOCK)
/// result:
/// - this prints a block of text directly to the console
pub fn console_qr(qrcode: &QrCode, quiet_zone: i32) {
    const ASCII_BL_BLOCK: &str = "  ";
    const ASCII_W_BLOCK: &str = "██";

    let x_zone = quiet_zone;
    let y_zone = quiet_zone;

    // paint top border -- y axis
    for _top_border in 0..y_zone {
        print!("{}", ASCII_BL_BLOCK);
        println!();
    }

    for y in 0..qrcode.size() {
        // paint left border -- x axis
        for _left_border in 0..x_zone {
            print!("{}", ASCII_BL_BLOCK);
        }

        // paint qr
        for x in 0..qrcode.size() {
            if qrcode.get_module(x, y) {
                print!("{}", ASCII_W_BLOCK);
            } else {
                print!("{}", ASCII_BL_BLOCK);
            }
        }

        // paint right border -- x axis
        for _right_border in 0..x_zone {
            print!("{}", ASCII_BL_BLOCK);
        }

        println!();
    }

    // paint bottom border -- y axis
    for _bottom_border in 0..y_zone {
        print!("{}", ASCII_BL_BLOCK);
        println!();
    }
}

pub fn make_image(
    qrcode: &QrCode,
    scale: i32,
    border_size: i32,
) -> ImageBuffer<LumaA<u8>, Vec<u8>> {
    make_image_export(qrcode, scale, border_size)
}

/// generates an svg string from a QrCode (output from the QR library)
///
/// * qrcode: &QrCode
///
pub fn make_svg(qrcode: &QrCode) -> String {
    to_svg_string_export(qrcode, 4)
}

/// saves an image to a file
///
/// * image: ImageBuffer<>
///
/// * save_file: file path to save the image into
pub fn save_image(
    image: &ImageBuffer<LumaA<u8>, Vec<u8>>,
    save_file: String,
) -> Result<(), image::ImageError> {
    save_image_export(image, save_file)
}

/// this error is returned when a potentially invalid combination of choices are made in the process
/// of building a string to embed as a QR code.
///
/// a recommendation is returned to the caller as a string to provide corrective action
#[derive(Debug, Clone)]
pub struct FormatError(String);

impl std::error::Error for FormatError {
    fn description(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
